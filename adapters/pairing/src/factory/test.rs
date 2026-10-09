#![allow(clippy::panic)]

use super::create_pairing;
use super::provides::{
    CreatePairingDepsOverrides, CreatePairingErrorReturn, CreatePairingParamsOverrides,
    CreatePairingPayload, MockIPairingAdapterFailureMode, MockIPairingConsumer, PairingConcrete,
    PrecompileEncoding, TargetGroupEncodingIdentifier, build_create_pairing_deps,
    build_create_pairing_params,
};

/// Contract: given `params.concrete` is `Bn254Arkworks`, its declared precompile encoding
///   is among `params.supported_encodings`, and its declared target-group encoding equals
///   `params.target_group_encoding`, the factory returns the success arm.
/// Arrange: `build_create_pairing_params` with only `concrete` overridden to `Bn254Arkworks`,
///   so `supported_encodings` holds the encoding of each concrete and `target_group_encoding`
///   is `Bn254V1`; deps hold `MockIPairingConsumer` with no override.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.is_ok()`.
#[test]
fn create_pairing_admits_the_bn254_arkworks_concrete() {
    // Arrange
    let deps =
        build_create_pairing_deps::<MockIPairingConsumer>(CreatePairingDepsOverrides::default());
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert!(result.is_ok());
}

/// Contract: given `params.concrete` is `Bn254Halo2curves`, its declared precompile encoding
///   is among `params.supported_encodings`, and its declared target-group encoding equals
///   `params.target_group_encoding`, the factory returns the success arm.
/// Arrange: `build_create_pairing_params` with only `concrete` overridden to `Bn254Halo2curves`,
///   so `supported_encodings` holds the encoding of each concrete and `target_group_encoding`
///   is `Bn254V1`; deps hold `MockIPairingConsumer` with no override.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.is_ok()`.
#[test]
fn create_pairing_admits_the_bn254_halo2curves_concrete() {
    // Arrange
    let deps =
        build_create_pairing_deps::<MockIPairingConsumer>(CreatePairingDepsOverrides::default());
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Halo2curves),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert!(result.is_ok());
}

/// Contract: given `params.concrete` is `Bls12381Arkworks`, its declared precompile encoding
///   is among `params.supported_encodings`, and its declared target-group encoding equals
///   `params.target_group_encoding`, the factory returns the success arm.
/// Arrange: `build_create_pairing_params` with only `concrete` overridden to `Bls12381Arkworks`,
///   so `supported_encodings` holds the encoding of each concrete and `target_group_encoding`
///   is `Bls12381V1`; deps hold `MockIPairingConsumer` with no override.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.is_ok()`.
#[test]
fn create_pairing_admits_the_bls12_381_arkworks_concrete() {
    // Arrange
    let deps =
        build_create_pairing_deps::<MockIPairingConsumer>(CreatePairingDepsOverrides::default());
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert!(result.is_ok());
}

/// Contract: given `params.concrete` is `Bls12381Halo2curves`, its declared precompile encoding
///   is among `params.supported_encodings`, and its declared target-group encoding equals
///   `params.target_group_encoding`, the factory returns the success arm.
/// Arrange: `build_create_pairing_params` with only `concrete` overridden to `Bls12381Halo2curves`,
///   so `supported_encodings` holds the encoding of each concrete and `target_group_encoding`
///   is `Bls12381V1`; deps hold `MockIPairingConsumer` with no override.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.is_ok()`.
#[test]
fn create_pairing_admits_the_bls12_381_halo2curves_concrete() {
    // Arrange
    let deps =
        build_create_pairing_deps::<MockIPairingConsumer>(CreatePairingDepsOverrides::default());
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Halo2curves),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert!(result.is_ok());
}

/// Contract: given `params.concrete` is `Mock(Succeeds)`, its declared precompile encoding
///   is among `params.supported_encodings`, and its declared target-group encoding equals
///   `params.target_group_encoding`, the factory returns the success arm.
/// Arrange: `build_create_pairing_params` with only `concrete` overridden to
///   `Mock(MockIPairingAdapterFailureMode::Succeeds)`, so `supported_encodings` holds the
///   encoding of each concrete and `target_group_encoding` is `Mock`; deps hold
///   `MockIPairingConsumer` with no override.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.is_ok()`.
#[test]
fn create_pairing_admits_the_mock_concrete() {
    // Arrange
    let deps =
        build_create_pairing_deps::<MockIPairingConsumer>(CreatePairingDepsOverrides::default());
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::Succeeds,
        )),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert!(result.is_ok());
}

/// Contract: given the `Bn254Arkworks` declared precompile encoding is not in
///   `params.supported_encodings`, the factory returns
///   `Err(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`.
/// Arrange: `build_create_pairing_params` with `concrete` overridden to `Bn254Arkworks` and
///   `supported_encodings` overridden to hold only `Eip2537`, which the variant does not
///   declare; `target_group_encoding` at the builder's `Bn254V1` default; deps with no override.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.err()` is `Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`.
#[test]
fn create_pairing_refuses_the_bn254_arkworks_concrete_whose_encoding_the_chain_does_not_deploy() {
    // Arrange
    let deps =
        build_create_pairing_deps::<MockIPairingConsumer>(CreatePairingDepsOverrides::default());
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        supported_encodings: Some(vec![PrecompileEncoding::Eip2537]),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)
    );
}

/// Contract: given the `Bn254Halo2curves` declared precompile encoding is not in
///   `params.supported_encodings`, the factory returns
///   `Err(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`.
/// Arrange: `build_create_pairing_params` with `concrete` overridden to `Bn254Halo2curves` and
///   `supported_encodings` overridden to hold only `Eip2537`, which the variant does not
///   declare; `target_group_encoding` at the builder's `Bn254V1` default; deps with no override.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.err()` is `Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`.
#[test]
fn create_pairing_refuses_the_bn254_halo2curves_concrete_whose_encoding_the_chain_does_not_deploy()
{
    // Arrange
    let deps =
        build_create_pairing_deps::<MockIPairingConsumer>(CreatePairingDepsOverrides::default());
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Halo2curves),
        supported_encodings: Some(vec![PrecompileEncoding::Eip2537]),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)
    );
}

/// Contract: given the `Bls12381Arkworks` declared precompile encoding is not in
///   `params.supported_encodings`, the factory returns
///   `Err(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`.
/// Arrange: `build_create_pairing_params` with `concrete` overridden to `Bls12381Arkworks` and
///   `supported_encodings` overridden to hold only `Eip196Eip197`, which the variant does not
///   declare; `target_group_encoding` at the builder's `Bls12381V1` default; deps with no
///   override.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.err()` is `Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`.
#[test]
fn create_pairing_refuses_the_bls12_381_arkworks_concrete_whose_encoding_the_chain_does_not_deploy()
{
    // Arrange
    let deps =
        build_create_pairing_deps::<MockIPairingConsumer>(CreatePairingDepsOverrides::default());
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        supported_encodings: Some(vec![PrecompileEncoding::Eip196Eip197]),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)
    );
}

/// Contract: given the `Bls12381Halo2curves` declared precompile encoding is not in
///   `params.supported_encodings`, the factory returns
///   `Err(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`.
/// Arrange: `build_create_pairing_params` with `concrete` overridden to `Bls12381Halo2curves`
///   and `supported_encodings` overridden to hold only `Eip196Eip197`, which the variant does
///   not declare; `target_group_encoding` at the builder's `Bls12381V1` default; deps with no
///   override.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.err()` is `Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`.
#[test]
fn create_pairing_refuses_the_bls12_381_halo2curves_concrete_whose_encoding_the_chain_does_not_deploy()
 {
    // Arrange
    let deps =
        build_create_pairing_deps::<MockIPairingConsumer>(CreatePairingDepsOverrides::default());
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Halo2curves),
        supported_encodings: Some(vec![PrecompileEncoding::Eip196Eip197]),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)
    );
}

/// Contract: given the `Mock` declared precompile encoding is not in
///   `params.supported_encodings`, the factory returns
///   `Err(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`.
/// Arrange: `build_create_pairing_params` with `concrete` overridden to
///   `Mock(MockIPairingAdapterFailureMode::Succeeds)` and `supported_encodings` overridden to
///   hold only `Eip196Eip197`, which the variant does not declare; `target_group_encoding` at
///   the builder's `Mock` default; deps with no override.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.err()` is `Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`.
#[test]
fn create_pairing_refuses_the_mock_concrete_whose_encoding_the_chain_does_not_deploy() {
    // Arrange
    let deps =
        build_create_pairing_deps::<MockIPairingConsumer>(CreatePairingDepsOverrides::default());
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::Succeeds,
        )),
        supported_encodings: Some(vec![PrecompileEncoding::Eip196Eip197]),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)
    );
}

/// Contract: given the `Bn254Arkworks` declared precompile encoding is in
///   `params.supported_encodings` and its declared target-group encoding differs from
///   `params.target_group_encoding`, the factory returns
///   `Err(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding)`.
/// Arrange: `build_create_pairing_params` with `concrete` overridden to `Bn254Arkworks` and
///   `target_group_encoding` overridden to `Bls12381V1`, which the variant does not declare;
///   `supported_encodings` at the builder's default; deps with no override.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.err()` is `Some(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding)`.
#[test]
fn create_pairing_refuses_the_bn254_arkworks_concrete_whose_target_group_encoding_the_suite_does_not_require()
 {
    // Arrange
    let deps =
        build_create_pairing_deps::<MockIPairingConsumer>(CreatePairingDepsOverrides::default());
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        target_group_encoding: Some(TargetGroupEncodingIdentifier::Bls12381V1),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding)
    );
}

/// Contract: given the `Bn254Halo2curves` declared precompile encoding is in
///   `params.supported_encodings` and its declared target-group encoding differs from
///   `params.target_group_encoding`, the factory returns
///   `Err(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding)`.
/// Arrange: `build_create_pairing_params` with `concrete` overridden to `Bn254Halo2curves` and
///   `target_group_encoding` overridden to `Bls12381V1`, which the variant does not declare;
///   `supported_encodings` at the builder's default; deps with no override.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.err()` is `Some(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding)`.
#[test]
fn create_pairing_refuses_the_bn254_halo2curves_concrete_whose_target_group_encoding_the_suite_does_not_require()
 {
    // Arrange
    let deps =
        build_create_pairing_deps::<MockIPairingConsumer>(CreatePairingDepsOverrides::default());
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Halo2curves),
        target_group_encoding: Some(TargetGroupEncodingIdentifier::Bls12381V1),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding)
    );
}

/// Contract: given the `Bls12381Arkworks` declared precompile encoding is in
///   `params.supported_encodings` and its declared target-group encoding differs from
///   `params.target_group_encoding`, the factory returns
///   `Err(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding)`.
/// Arrange: `build_create_pairing_params` with `concrete` overridden to `Bls12381Arkworks` and
///   `target_group_encoding` overridden to `Bn254V1`, which the variant does not declare;
///   `supported_encodings` at the builder's default; deps with no override.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.err()` is `Some(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding)`.
#[test]
fn create_pairing_refuses_the_bls12_381_arkworks_concrete_whose_target_group_encoding_the_suite_does_not_require()
 {
    // Arrange
    let deps =
        build_create_pairing_deps::<MockIPairingConsumer>(CreatePairingDepsOverrides::default());
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        target_group_encoding: Some(TargetGroupEncodingIdentifier::Bn254V1),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding)
    );
}

/// Contract: given the `Bls12381Halo2curves` declared precompile encoding is in
///   `params.supported_encodings` and its declared target-group encoding differs from
///   `params.target_group_encoding`, the factory returns
///   `Err(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding)`.
/// Arrange: `build_create_pairing_params` with `concrete` overridden to `Bls12381Halo2curves`
///   and `target_group_encoding` overridden to `Bn254V1`, which the variant does not declare;
///   `supported_encodings` at the builder's default; deps with no override.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.err()` is `Some(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding)`.
#[test]
fn create_pairing_refuses_the_bls12_381_halo2curves_concrete_whose_target_group_encoding_the_suite_does_not_require()
 {
    // Arrange
    let deps =
        build_create_pairing_deps::<MockIPairingConsumer>(CreatePairingDepsOverrides::default());
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Halo2curves),
        target_group_encoding: Some(TargetGroupEncodingIdentifier::Bn254V1),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding)
    );
}

/// Contract: given the `Mock` declared precompile encoding is in
///   `params.supported_encodings` and its declared target-group encoding differs from
///   `params.target_group_encoding`, the factory returns
///   `Err(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding)`.
/// Arrange: `build_create_pairing_params` with `concrete` overridden to
///   `Mock(MockIPairingAdapterFailureMode::Succeeds)` and `target_group_encoding` overridden
///   to `Bn254V1`, which the variant does not declare; `supported_encodings` at the builder's
///   default; deps with no override.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.err()` is `Some(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding)`.
#[test]
fn create_pairing_refuses_the_mock_concrete_whose_target_group_encoding_the_suite_does_not_require()
{
    // Arrange
    let deps =
        build_create_pairing_deps::<MockIPairingConsumer>(CreatePairingDepsOverrides::default());
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::Succeeds,
        )),
        target_group_encoding: Some(TargetGroupEncodingIdentifier::Bn254V1),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreatePairingErrorReturn::UnsupportedTargetGroupEncoding)
    );
}

/// Contract: given both the `Bn254Arkworks` declared precompile encoding is not in
///   `params.supported_encodings` and its declared target-group encoding differs from
///   `params.target_group_encoding`, the factory returns
///   `Err(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`, the encoding admission
///   preceding the target-group admission.
/// Arrange: `build_create_pairing_params` with `concrete` overridden to `Bn254Arkworks`,
///   `supported_encodings` overridden to hold only `Eip2537`, and `target_group_encoding`
///   overridden to `Bls12381V1`, so both admissions fail; deps with no override.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.err()` is `Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`.
#[test]
fn create_pairing_refuses_the_encoding_before_the_target_group_encoding_for_the_bn254_arkworks_concrete()
 {
    // Arrange
    let deps =
        build_create_pairing_deps::<MockIPairingConsumer>(CreatePairingDepsOverrides::default());
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        supported_encodings: Some(vec![PrecompileEncoding::Eip2537]),
        target_group_encoding: Some(TargetGroupEncodingIdentifier::Bls12381V1),
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)
    );
}

/// Contract: given both the `Bn254Halo2curves` declared precompile encoding is not in
///   `params.supported_encodings` and its declared target-group encoding differs from
///   `params.target_group_encoding`, the factory returns
///   `Err(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`, the encoding admission
///   preceding the target-group admission.
/// Arrange: `build_create_pairing_params` with `concrete` overridden to `Bn254Halo2curves`,
///   `supported_encodings` overridden to hold only `Eip2537`, and `target_group_encoding`
///   overridden to `Bls12381V1`, so both admissions fail; deps with no override.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.err()` is `Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`.
#[test]
fn create_pairing_refuses_the_encoding_before_the_target_group_encoding_for_the_bn254_halo2curves_concrete()
 {
    // Arrange
    let deps =
        build_create_pairing_deps::<MockIPairingConsumer>(CreatePairingDepsOverrides::default());
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Halo2curves),
        supported_encodings: Some(vec![PrecompileEncoding::Eip2537]),
        target_group_encoding: Some(TargetGroupEncodingIdentifier::Bls12381V1),
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)
    );
}

/// Contract: given both the `Bls12381Arkworks` declared precompile encoding is not in
///   `params.supported_encodings` and its declared target-group encoding differs from
///   `params.target_group_encoding`, the factory returns
///   `Err(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`, the encoding admission
///   preceding the target-group admission.
/// Arrange: `build_create_pairing_params` with `concrete` overridden to `Bls12381Arkworks`,
///   `supported_encodings` overridden to hold only `Eip196Eip197`, and `target_group_encoding`
///   overridden to `Bn254V1`, so both admissions fail; deps with no override.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.err()` is `Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`.
#[test]
fn create_pairing_refuses_the_encoding_before_the_target_group_encoding_for_the_bls12_381_arkworks_concrete()
 {
    // Arrange
    let deps =
        build_create_pairing_deps::<MockIPairingConsumer>(CreatePairingDepsOverrides::default());
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        supported_encodings: Some(vec![PrecompileEncoding::Eip196Eip197]),
        target_group_encoding: Some(TargetGroupEncodingIdentifier::Bn254V1),
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)
    );
}

/// Contract: given both the `Bls12381Halo2curves` declared precompile encoding is not in
///   `params.supported_encodings` and its declared target-group encoding differs from
///   `params.target_group_encoding`, the factory returns
///   `Err(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`, the encoding admission
///   preceding the target-group admission.
/// Arrange: `build_create_pairing_params` with `concrete` overridden to `Bls12381Halo2curves`,
///   `supported_encodings` overridden to hold only `Eip196Eip197`, and `target_group_encoding`
///   overridden to `Bn254V1`, so both admissions fail; deps with no override.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.err()` is `Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`.
#[test]
fn create_pairing_refuses_the_encoding_before_the_target_group_encoding_for_the_bls12_381_halo2curves_concrete()
 {
    // Arrange
    let deps =
        build_create_pairing_deps::<MockIPairingConsumer>(CreatePairingDepsOverrides::default());
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Halo2curves),
        supported_encodings: Some(vec![PrecompileEncoding::Eip196Eip197]),
        target_group_encoding: Some(TargetGroupEncodingIdentifier::Bn254V1),
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)
    );
}

/// Contract: given both the `Mock` declared precompile encoding is not in
///   `params.supported_encodings` and its declared target-group encoding differs from
///   `params.target_group_encoding`, the factory returns
///   `Err(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`, the encoding admission
///   preceding the target-group admission.
/// Arrange: `build_create_pairing_params` with `concrete` overridden to
///   `Mock(MockIPairingAdapterFailureMode::Succeeds)`, `supported_encodings` overridden to
///   hold only `Eip196Eip197`, and `target_group_encoding` overridden to `Bn254V1`, so both
///   admissions fail; deps with no override.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.err()` is `Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)`.
#[test]
fn create_pairing_refuses_the_encoding_before_the_target_group_encoding_for_the_mock_concrete() {
    // Arrange
    let deps =
        build_create_pairing_deps::<MockIPairingConsumer>(CreatePairingDepsOverrides::default());
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::Succeeds,
        )),
        supported_encodings: Some(vec![PrecompileEncoding::Eip196Eip197]),
        target_group_encoding: Some(TargetGroupEncodingIdentifier::Bn254V1),
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreatePairingErrorReturn::UnsupportedPrecompileEncoding)
    );
}
