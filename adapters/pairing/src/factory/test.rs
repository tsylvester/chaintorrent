use super::create_pairing;
use super::provides::{
    CreatePairingErrorReturn, CreatePairingParamsOverrides, CreatePairingPayload,
    MockIPairingAdapterFailureMode, MockIPairingConsumer, PairingConcrete, PairingCurve,
    PairingDeclarationOverrides, PrecompileEncoding, TargetGroupEncodingIdentifier,
    VerifierGroupArithmetic, build_create_pairing_deps, build_create_pairing_params,
    build_pairing_declaration,
};
use crate::admit_pairing_composition::provides::{
    AdmitPairingCompositionErrorReturn, AdmitPairingCompositionFn, PairingDisagreement,
    build_admit_pairing_composition_deps, mock_admit_pairing_composition,
};
use crate::lookup_pairing_declaration::provides::{
    LookupPairingDeclarationFn, LookupPairingDeclarationSuccessReturn,
    mock_lookup_pairing_declaration,
};

/// Contract: given `params.concrete` is `Bn254Arkworks` and the composition admission
///   returns `Ok`, the factory calls its declaration lookup once for `params.concrete`,
///   constructs the arm's concrete, hands the consumer the adapter and the declaration
///   the lookup returned in `ConsumePairingPayload`, and returns `Ok` whose `output` is
///   that declaration — the consumer's return of `payload.declaration` — whose
///   `concrete` equals `params.concrete`.
/// Arrange: `build_create_pairing_params` with only `concrete` overridden to
///   `Bn254Arkworks`; a declaration lookup the block declares, typed
///   `LookupPairingDeclarationFn`, returning `Ok(LookupPairingDeclarationSuccessReturn {
///   declaration })` whose `declaration` `build_pairing_declaration` builds with
///   `concrete` overridden to the `concrete` the call's `LookupPairingDeclarationParams`
///   carries, so the assert proves the factory called the lookup with `params.concrete`
///   and handed its return to the consumer, and a factory that looks up a sibling or
///   skips the lookup fails; deps built with `MockIPairingConsumer`, that lookup, the
///   official admission mock returning `Ok`, and
///   `build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, _)`.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.ok().map(|success| success.output)` equals
///   `build_pairing_declaration` with `concrete` overridden to `Bn254Arkworks` — the
///   payload declaration whose `concrete` equals `params.concrete`.
#[test]
fn create_pairing_admits_the_bn254_arkworks_concrete() {
    // Arrange
    let lookup: LookupPairingDeclarationFn = |_deps, params, _payload| {
        Ok(LookupPairingDeclarationSuccessReturn {
            declaration: build_pairing_declaration(PairingDeclarationOverrides {
                concrete: Some(params.concrete),
                ..Default::default()
            }),
        })
    };
    let deps = build_create_pairing_deps(
        MockIPairingConsumer,
        lookup,
        mock_admit_pairing_composition,
        build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, Default::default()),
        Default::default(),
    );
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.ok().map(|success| success.output),
        Some(build_pairing_declaration(PairingDeclarationOverrides {
            concrete: Some(PairingConcrete::Bn254Arkworks),
            ..Default::default()
        }))
    );
}

/// Contract: given `params.concrete` is `Bn254Halo2curves` and the composition admission
///   returns `Ok`, the factory calls its declaration lookup once for `params.concrete`,
///   constructs the arm's concrete, hands the consumer the adapter and the declaration
///   the lookup returned in `ConsumePairingPayload`, and returns `Ok` whose `output` is
///   that declaration — the consumer's return of `payload.declaration` — whose
///   `concrete` equals `params.concrete`.
/// Arrange: `build_create_pairing_params` with only `concrete` overridden to
///   `Bn254Halo2curves`; a declaration lookup the block declares, typed
///   `LookupPairingDeclarationFn`, returning `Ok(LookupPairingDeclarationSuccessReturn {
///   declaration })` whose `declaration` `build_pairing_declaration` builds with
///   `concrete` overridden to the `concrete` the call's `LookupPairingDeclarationParams`
///   carries, so the assert proves the factory called the lookup with `params.concrete`
///   and handed its return to the consumer, and a factory that looks up a sibling or
///   skips the lookup fails; deps built with `MockIPairingConsumer`, that lookup, the
///   official admission mock returning `Ok`, and
///   `build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, _)`.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.ok().map(|success| success.output)` equals
///   `build_pairing_declaration` with `concrete` overridden to `Bn254Halo2curves` — the
///   payload declaration whose `concrete` equals `params.concrete`.
#[test]
fn create_pairing_admits_the_bn254_halo2curves_concrete() {
    // Arrange
    let lookup: LookupPairingDeclarationFn = |_deps, params, _payload| {
        Ok(LookupPairingDeclarationSuccessReturn {
            declaration: build_pairing_declaration(PairingDeclarationOverrides {
                concrete: Some(params.concrete),
                ..Default::default()
            }),
        })
    };
    let deps = build_create_pairing_deps(
        MockIPairingConsumer,
        lookup,
        mock_admit_pairing_composition,
        build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, Default::default()),
        Default::default(),
    );
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Halo2curves),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.ok().map(|success| success.output),
        Some(build_pairing_declaration(PairingDeclarationOverrides {
            concrete: Some(PairingConcrete::Bn254Halo2curves),
            ..Default::default()
        }))
    );
}

/// Contract: given `params.concrete` is `Bls12381Arkworks` and the composition admission
///   returns `Ok`, the factory calls its declaration lookup once for `params.concrete`,
///   constructs the arm's concrete, hands the consumer the adapter and the declaration
///   the lookup returned in `ConsumePairingPayload`, and returns `Ok` whose `output` is
///   that declaration — the consumer's return of `payload.declaration` — whose
///   `concrete` equals `params.concrete`.
/// Arrange: `build_create_pairing_params` with only `concrete` overridden to
///   `Bls12381Arkworks`; a declaration lookup the block declares, typed
///   `LookupPairingDeclarationFn`, returning `Ok(LookupPairingDeclarationSuccessReturn {
///   declaration })` whose `declaration` `build_pairing_declaration` builds with
///   `concrete` overridden to the `concrete` the call's `LookupPairingDeclarationParams`
///   carries, so the assert proves the factory called the lookup with `params.concrete`
///   and handed its return to the consumer, and a factory that looks up a sibling or
///   skips the lookup fails; deps built with `MockIPairingConsumer`, that lookup, the
///   official admission mock returning `Ok`, and
///   `build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, _)`.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.ok().map(|success| success.output)` equals
///   `build_pairing_declaration` with `concrete` overridden to `Bls12381Arkworks` — the
///   payload declaration whose `concrete` equals `params.concrete`.
#[test]
fn create_pairing_admits_the_bls12_381_arkworks_concrete() {
    // Arrange
    let lookup: LookupPairingDeclarationFn = |_deps, params, _payload| {
        Ok(LookupPairingDeclarationSuccessReturn {
            declaration: build_pairing_declaration(PairingDeclarationOverrides {
                concrete: Some(params.concrete),
                ..Default::default()
            }),
        })
    };
    let deps = build_create_pairing_deps(
        MockIPairingConsumer,
        lookup,
        mock_admit_pairing_composition,
        build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, Default::default()),
        Default::default(),
    );
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.ok().map(|success| success.output),
        Some(build_pairing_declaration(PairingDeclarationOverrides {
            concrete: Some(PairingConcrete::Bls12381Arkworks),
            ..Default::default()
        }))
    );
}

/// Contract: given `params.concrete` is `Bls12381Halo2curves` and the composition
///   admission returns `Ok`, the factory calls its declaration lookup once for
///   `params.concrete`, constructs the arm's concrete, hands the consumer the adapter
///   and the declaration the lookup returned in `ConsumePairingPayload`, and returns
///   `Ok` whose `output` is that declaration — the consumer's return of
///   `payload.declaration` — whose `concrete` equals `params.concrete`.
/// Arrange: `build_create_pairing_params` with only `concrete` overridden to
///   `Bls12381Halo2curves`; a declaration lookup the block declares, typed
///   `LookupPairingDeclarationFn`, returning `Ok(LookupPairingDeclarationSuccessReturn {
///   declaration })` whose `declaration` `build_pairing_declaration` builds with
///   `concrete` overridden to the `concrete` the call's `LookupPairingDeclarationParams`
///   carries, so the assert proves the factory called the lookup with `params.concrete`
///   and handed its return to the consumer, and a factory that looks up a sibling or
///   skips the lookup fails; deps built with `MockIPairingConsumer`, that lookup, the
///   official admission mock returning `Ok`, and
///   `build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, _)`.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.ok().map(|success| success.output)` equals
///   `build_pairing_declaration` with `concrete` overridden to `Bls12381Halo2curves` —
///   the payload declaration whose `concrete` equals `params.concrete`.
#[test]
fn create_pairing_admits_the_bls12_381_halo2curves_concrete() {
    // Arrange
    let lookup: LookupPairingDeclarationFn = |_deps, params, _payload| {
        Ok(LookupPairingDeclarationSuccessReturn {
            declaration: build_pairing_declaration(PairingDeclarationOverrides {
                concrete: Some(params.concrete),
                ..Default::default()
            }),
        })
    };
    let deps = build_create_pairing_deps(
        MockIPairingConsumer,
        lookup,
        mock_admit_pairing_composition,
        build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, Default::default()),
        Default::default(),
    );
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Halo2curves),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.ok().map(|success| success.output),
        Some(build_pairing_declaration(PairingDeclarationOverrides {
            concrete: Some(PairingConcrete::Bls12381Halo2curves),
            ..Default::default()
        }))
    );
}

/// Contract: given `params.concrete` is `Mock(Succeeds)` and the composition admission
///   returns `Ok`, the factory calls its declaration lookup once for `params.concrete`,
///   constructs the arm's concrete, hands the consumer the adapter and the declaration
///   the lookup returned in `ConsumePairingPayload`, and returns `Ok` whose `output` is
///   that declaration — the consumer's return of `payload.declaration` — whose
///   `concrete` equals `params.concrete`.
/// Arrange: `build_create_pairing_params` with only `concrete` overridden to
///   `Mock(MockIPairingAdapterFailureMode::Succeeds)`; a declaration lookup the block
///   declares, typed `LookupPairingDeclarationFn`, returning
///   `Ok(LookupPairingDeclarationSuccessReturn { declaration })` whose `declaration`
///   `build_pairing_declaration` builds with `concrete` overridden to the `concrete` the
///   call's `LookupPairingDeclarationParams` carries, so the assert proves the factory
///   called the lookup with `params.concrete` and handed its return to the consumer, and
///   a factory that looks up a sibling or skips the lookup fails; deps built with
///   `MockIPairingConsumer`, that lookup, the official admission mock returning `Ok`,
///   and `build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, _)`.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.ok().map(|success| success.output)` equals
///   `build_pairing_declaration` with `concrete` overridden to
///   `Mock(MockIPairingAdapterFailureMode::Succeeds)` — the payload declaration whose
///   `concrete` equals `params.concrete`.
#[test]
fn create_pairing_admits_the_mock_concrete() {
    // Arrange
    let lookup: LookupPairingDeclarationFn = |_deps, params, _payload| {
        Ok(LookupPairingDeclarationSuccessReturn {
            declaration: build_pairing_declaration(PairingDeclarationOverrides {
                concrete: Some(params.concrete),
                ..Default::default()
            }),
        })
    };
    let deps = build_create_pairing_deps(
        MockIPairingConsumer,
        lookup,
        mock_admit_pairing_composition,
        build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, Default::default()),
        Default::default(),
    );
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::Succeeds,
        )),
        ..Default::default()
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.ok().map(|success| success.output),
        Some(build_pairing_declaration(PairingDeclarationOverrides {
            concrete: Some(PairingConcrete::Mock(
                MockIPairingAdapterFailureMode::Succeeds,
            )),
            ..Default::default()
        }))
    );
}

/// Contract: given the composition admission refuses `Bn254Arkworks`, the factory
///   returns `Err(CreatePairingErrorReturn::Composition(error))` carrying the
///   admission's error whole, the admission having been called with `params.concrete`
///   as the identifier and the four `required_*` values `params` carries.
/// Arrange: a composition admission the block declares, typed `AdmitPairingCompositionFn`,
///   returning `Err(Incompatible { identifier: params.identifier, disagreements })`
///   whose `disagreements` restates what the factory passed; `build_create_pairing_params`
///   with `concrete` overridden to `Bn254Arkworks` and the four `required_*` overridden
///   to the values of the curve the variant is not of; deps built with
///   `MockIPairingConsumer`, `mock_lookup_pairing_declaration` as the factory's own
///   lookup handle, that admission, and
///   `build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, _)`, the
///   lookup unreached and the consumer uncalled on the refusal.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.err()` is the interaction spec's outcome as a literal.
#[test]
fn create_pairing_returns_the_composition_refusal_whole_for_the_bn254_arkworks_concrete() {
    // Arrange
    let admission: AdmitPairingCompositionFn = |_deps, params, _payload| {
        Err(AdmitPairingCompositionErrorReturn::Incompatible {
            identifier: params.identifier,
            disagreements: vec![
                PairingDisagreement::Curve {
                    declared: PairingCurve::Bn254,
                    required: params.required_curve,
                },
                PairingDisagreement::VerifierGroupArithmetic {
                    declared: VerifierGroupArithmetic::FirstGroupOnly,
                    required: params.required_verifier_group_arithmetic,
                },
                PairingDisagreement::PrecompileEncoding {
                    declared: PrecompileEncoding::Eip196Eip197,
                    required: params.required_precompile_encodings,
                },
                PairingDisagreement::TargetGroupEncoding {
                    declared: TargetGroupEncodingIdentifier::Bn254V1,
                    required: params.required_target_group_encoding,
                },
            ],
        })
    };
    let deps = build_create_pairing_deps(
        MockIPairingConsumer,
        mock_lookup_pairing_declaration,
        admission,
        build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, Default::default()),
        Default::default(),
    );
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Arkworks),
        required_curve: Some(PairingCurve::Bls12381),
        required_verifier_group_arithmetic: Some(VerifierGroupArithmetic::BothGroups),
        required_precompile_encodings: Some(vec![PrecompileEncoding::Eip2537]),
        required_target_group_encoding: Some(TargetGroupEncodingIdentifier::Bls12381V1),
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreatePairingErrorReturn::Composition(
            AdmitPairingCompositionErrorReturn::Incompatible {
                identifier: PairingConcrete::Bn254Arkworks,
                disagreements: vec![
                    PairingDisagreement::Curve {
                        declared: PairingCurve::Bn254,
                        required: PairingCurve::Bls12381,
                    },
                    PairingDisagreement::VerifierGroupArithmetic {
                        declared: VerifierGroupArithmetic::FirstGroupOnly,
                        required: VerifierGroupArithmetic::BothGroups,
                    },
                    PairingDisagreement::PrecompileEncoding {
                        declared: PrecompileEncoding::Eip196Eip197,
                        required: vec![PrecompileEncoding::Eip2537],
                    },
                    PairingDisagreement::TargetGroupEncoding {
                        declared: TargetGroupEncodingIdentifier::Bn254V1,
                        required: TargetGroupEncodingIdentifier::Bls12381V1,
                    },
                ],
            }
        ))
    );
}

/// Contract: given the composition admission refuses `Bn254Halo2curves`, the factory
///   returns `Err(CreatePairingErrorReturn::Composition(error))` carrying the
///   admission's error whole, the admission having been called with `params.concrete`
///   as the identifier and the four `required_*` values `params` carries.
/// Arrange: a composition admission the block declares, typed `AdmitPairingCompositionFn`,
///   returning `Err(Incompatible { identifier: params.identifier, disagreements })`
///   whose `disagreements` restates what the factory passed; `build_create_pairing_params`
///   with `concrete` overridden to `Bn254Halo2curves` and the four `required_*` overridden
///   to the values of the curve the variant is not of; deps built with
///   `MockIPairingConsumer`, `mock_lookup_pairing_declaration` as the factory's own
///   lookup handle, that admission, and
///   `build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, _)`, the
///   lookup unreached and the consumer uncalled on the refusal.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.err()` is the interaction spec's outcome as a literal.
#[test]
fn create_pairing_returns_the_composition_refusal_whole_for_the_bn254_halo2curves_concrete() {
    // Arrange
    let admission: AdmitPairingCompositionFn = |_deps, params, _payload| {
        Err(AdmitPairingCompositionErrorReturn::Incompatible {
            identifier: params.identifier,
            disagreements: vec![
                PairingDisagreement::Curve {
                    declared: PairingCurve::Bn254,
                    required: params.required_curve,
                },
                PairingDisagreement::VerifierGroupArithmetic {
                    declared: VerifierGroupArithmetic::FirstGroupOnly,
                    required: params.required_verifier_group_arithmetic,
                },
                PairingDisagreement::PrecompileEncoding {
                    declared: PrecompileEncoding::Eip196Eip197,
                    required: params.required_precompile_encodings,
                },
                PairingDisagreement::TargetGroupEncoding {
                    declared: TargetGroupEncodingIdentifier::Bn254V1,
                    required: params.required_target_group_encoding,
                },
            ],
        })
    };
    let deps = build_create_pairing_deps(
        MockIPairingConsumer,
        mock_lookup_pairing_declaration,
        admission,
        build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, Default::default()),
        Default::default(),
    );
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bn254Halo2curves),
        required_curve: Some(PairingCurve::Bls12381),
        required_verifier_group_arithmetic: Some(VerifierGroupArithmetic::BothGroups),
        required_precompile_encodings: Some(vec![PrecompileEncoding::Eip2537]),
        required_target_group_encoding: Some(TargetGroupEncodingIdentifier::Bls12381V1),
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreatePairingErrorReturn::Composition(
            AdmitPairingCompositionErrorReturn::Incompatible {
                identifier: PairingConcrete::Bn254Halo2curves,
                disagreements: vec![
                    PairingDisagreement::Curve {
                        declared: PairingCurve::Bn254,
                        required: PairingCurve::Bls12381,
                    },
                    PairingDisagreement::VerifierGroupArithmetic {
                        declared: VerifierGroupArithmetic::FirstGroupOnly,
                        required: VerifierGroupArithmetic::BothGroups,
                    },
                    PairingDisagreement::PrecompileEncoding {
                        declared: PrecompileEncoding::Eip196Eip197,
                        required: vec![PrecompileEncoding::Eip2537],
                    },
                    PairingDisagreement::TargetGroupEncoding {
                        declared: TargetGroupEncodingIdentifier::Bn254V1,
                        required: TargetGroupEncodingIdentifier::Bls12381V1,
                    },
                ],
            }
        ))
    );
}

/// Contract: given the composition admission refuses `Bls12381Arkworks`, the factory
///   returns `Err(CreatePairingErrorReturn::Composition(error))` carrying the
///   admission's error whole, the admission having been called with `params.concrete`
///   as the identifier and the four `required_*` values `params` carries.
/// Arrange: a composition admission the block declares, typed `AdmitPairingCompositionFn`,
///   returning `Err(Incompatible { identifier: params.identifier, disagreements })`
///   whose `disagreements` restates what the factory passed; `build_create_pairing_params`
///   with `concrete` overridden to `Bls12381Arkworks` and the four `required_*` overridden
///   to the values of the curve the variant is not of; deps built with
///   `MockIPairingConsumer`, `mock_lookup_pairing_declaration` as the factory's own
///   lookup handle, that admission, and
///   `build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, _)`, the
///   lookup unreached and the consumer uncalled on the refusal.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.err()` is the interaction spec's outcome as a literal.
#[test]
fn create_pairing_returns_the_composition_refusal_whole_for_the_bls12_381_arkworks_concrete() {
    // Arrange
    let admission: AdmitPairingCompositionFn = |_deps, params, _payload| {
        Err(AdmitPairingCompositionErrorReturn::Incompatible {
            identifier: params.identifier,
            disagreements: vec![
                PairingDisagreement::Curve {
                    declared: PairingCurve::Bn254,
                    required: params.required_curve,
                },
                PairingDisagreement::VerifierGroupArithmetic {
                    declared: VerifierGroupArithmetic::FirstGroupOnly,
                    required: params.required_verifier_group_arithmetic,
                },
                PairingDisagreement::PrecompileEncoding {
                    declared: PrecompileEncoding::Eip196Eip197,
                    required: params.required_precompile_encodings,
                },
                PairingDisagreement::TargetGroupEncoding {
                    declared: TargetGroupEncodingIdentifier::Bn254V1,
                    required: params.required_target_group_encoding,
                },
            ],
        })
    };
    let deps = build_create_pairing_deps(
        MockIPairingConsumer,
        mock_lookup_pairing_declaration,
        admission,
        build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, Default::default()),
        Default::default(),
    );
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Arkworks),
        required_curve: Some(PairingCurve::Bn254),
        required_verifier_group_arithmetic: Some(VerifierGroupArithmetic::FirstGroupOnly),
        required_precompile_encodings: Some(vec![PrecompileEncoding::Eip196Eip197]),
        required_target_group_encoding: Some(TargetGroupEncodingIdentifier::Bn254V1),
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreatePairingErrorReturn::Composition(
            AdmitPairingCompositionErrorReturn::Incompatible {
                identifier: PairingConcrete::Bls12381Arkworks,
                disagreements: vec![
                    PairingDisagreement::Curve {
                        declared: PairingCurve::Bn254,
                        required: PairingCurve::Bn254,
                    },
                    PairingDisagreement::VerifierGroupArithmetic {
                        declared: VerifierGroupArithmetic::FirstGroupOnly,
                        required: VerifierGroupArithmetic::FirstGroupOnly,
                    },
                    PairingDisagreement::PrecompileEncoding {
                        declared: PrecompileEncoding::Eip196Eip197,
                        required: vec![PrecompileEncoding::Eip196Eip197],
                    },
                    PairingDisagreement::TargetGroupEncoding {
                        declared: TargetGroupEncodingIdentifier::Bn254V1,
                        required: TargetGroupEncodingIdentifier::Bn254V1,
                    },
                ],
            }
        ))
    );
}

/// Contract: given the composition admission refuses `Bls12381Halo2curves`, the factory
///   returns `Err(CreatePairingErrorReturn::Composition(error))` carrying the
///   admission's error whole, the admission having been called with `params.concrete`
///   as the identifier and the four `required_*` values `params` carries.
/// Arrange: a composition admission the block declares, typed `AdmitPairingCompositionFn`,
///   returning `Err(Incompatible { identifier: params.identifier, disagreements })`
///   whose `disagreements` restates what the factory passed; `build_create_pairing_params`
///   with `concrete` overridden to `Bls12381Halo2curves` and the four `required_*`
///   overridden to the values of the curve the variant is not of; deps built with
///   `MockIPairingConsumer`, `mock_lookup_pairing_declaration` as the factory's own
///   lookup handle, that admission, and
///   `build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, _)`, the
///   lookup unreached and the consumer uncalled on the refusal.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.err()` is the interaction spec's outcome as a literal.
#[test]
fn create_pairing_returns_the_composition_refusal_whole_for_the_bls12_381_halo2curves_concrete() {
    // Arrange
    let admission: AdmitPairingCompositionFn = |_deps, params, _payload| {
        Err(AdmitPairingCompositionErrorReturn::Incompatible {
            identifier: params.identifier,
            disagreements: vec![
                PairingDisagreement::Curve {
                    declared: PairingCurve::Bn254,
                    required: params.required_curve,
                },
                PairingDisagreement::VerifierGroupArithmetic {
                    declared: VerifierGroupArithmetic::FirstGroupOnly,
                    required: params.required_verifier_group_arithmetic,
                },
                PairingDisagreement::PrecompileEncoding {
                    declared: PrecompileEncoding::Eip196Eip197,
                    required: params.required_precompile_encodings,
                },
                PairingDisagreement::TargetGroupEncoding {
                    declared: TargetGroupEncodingIdentifier::Bn254V1,
                    required: params.required_target_group_encoding,
                },
            ],
        })
    };
    let deps = build_create_pairing_deps(
        MockIPairingConsumer,
        mock_lookup_pairing_declaration,
        admission,
        build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, Default::default()),
        Default::default(),
    );
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Bls12381Halo2curves),
        required_curve: Some(PairingCurve::Bn254),
        required_verifier_group_arithmetic: Some(VerifierGroupArithmetic::FirstGroupOnly),
        required_precompile_encodings: Some(vec![PrecompileEncoding::Eip196Eip197]),
        required_target_group_encoding: Some(TargetGroupEncodingIdentifier::Bn254V1),
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreatePairingErrorReturn::Composition(
            AdmitPairingCompositionErrorReturn::Incompatible {
                identifier: PairingConcrete::Bls12381Halo2curves,
                disagreements: vec![
                    PairingDisagreement::Curve {
                        declared: PairingCurve::Bn254,
                        required: PairingCurve::Bn254,
                    },
                    PairingDisagreement::VerifierGroupArithmetic {
                        declared: VerifierGroupArithmetic::FirstGroupOnly,
                        required: VerifierGroupArithmetic::FirstGroupOnly,
                    },
                    PairingDisagreement::PrecompileEncoding {
                        declared: PrecompileEncoding::Eip196Eip197,
                        required: vec![PrecompileEncoding::Eip196Eip197],
                    },
                    PairingDisagreement::TargetGroupEncoding {
                        declared: TargetGroupEncodingIdentifier::Bn254V1,
                        required: TargetGroupEncodingIdentifier::Bn254V1,
                    },
                ],
            }
        ))
    );
}

/// Contract: given the composition admission refuses `Mock(Succeeds)`, the factory
///   returns `Err(CreatePairingErrorReturn::Composition(error))` carrying the
///   admission's error whole, the admission having been called with `params.concrete`
///   as the identifier and the four `required_*` values `params` carries.
/// Arrange: a composition admission the block declares, typed `AdmitPairingCompositionFn`,
///   returning `Err(Incompatible { identifier: params.identifier, disagreements })`
///   whose `disagreements` restates what the factory passed; `build_create_pairing_params`
///   with `concrete` overridden to `Mock(MockIPairingAdapterFailureMode::Succeeds)` and
///   the four `required_*` overridden to the values of the curve the variant is not of;
///   deps built with `MockIPairingConsumer`, `mock_lookup_pairing_declaration` as the
///   factory's own lookup handle, that admission, and
///   `build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, _)`, the
///   lookup unreached and the consumer uncalled on the refusal.
/// Act:     `create_pairing(&deps, params, CreatePairingPayload)`.
/// Assert:  `result.err()` is the interaction spec's outcome as a literal.
#[test]
fn create_pairing_returns_the_composition_refusal_whole_for_the_mock_concrete() {
    // Arrange
    let admission: AdmitPairingCompositionFn = |_deps, params, _payload| {
        Err(AdmitPairingCompositionErrorReturn::Incompatible {
            identifier: params.identifier,
            disagreements: vec![
                PairingDisagreement::Curve {
                    declared: PairingCurve::Bn254,
                    required: params.required_curve,
                },
                PairingDisagreement::VerifierGroupArithmetic {
                    declared: VerifierGroupArithmetic::FirstGroupOnly,
                    required: params.required_verifier_group_arithmetic,
                },
                PairingDisagreement::PrecompileEncoding {
                    declared: PrecompileEncoding::Eip196Eip197,
                    required: params.required_precompile_encodings,
                },
                PairingDisagreement::TargetGroupEncoding {
                    declared: TargetGroupEncodingIdentifier::Bn254V1,
                    required: params.required_target_group_encoding,
                },
            ],
        })
    };
    let deps = build_create_pairing_deps(
        MockIPairingConsumer,
        mock_lookup_pairing_declaration,
        admission,
        build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, Default::default()),
        Default::default(),
    );
    let params = build_create_pairing_params(CreatePairingParamsOverrides {
        concrete: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::Succeeds,
        )),
        required_curve: Some(PairingCurve::Bls12381),
        required_verifier_group_arithmetic: Some(VerifierGroupArithmetic::BothGroups),
        required_precompile_encodings: Some(vec![PrecompileEncoding::Eip2537]),
        required_target_group_encoding: Some(TargetGroupEncodingIdentifier::Bls12381V1),
    });

    // Act
    let result = create_pairing(&deps, params, CreatePairingPayload);

    // Assert
    assert_eq!(
        result.err(),
        Some(CreatePairingErrorReturn::Composition(
            AdmitPairingCompositionErrorReturn::Incompatible {
                identifier: PairingConcrete::Mock(MockIPairingAdapterFailureMode::Succeeds),
                disagreements: vec![
                    PairingDisagreement::Curve {
                        declared: PairingCurve::Bn254,
                        required: PairingCurve::Bls12381,
                    },
                    PairingDisagreement::VerifierGroupArithmetic {
                        declared: VerifierGroupArithmetic::FirstGroupOnly,
                        required: VerifierGroupArithmetic::BothGroups,
                    },
                    PairingDisagreement::PrecompileEncoding {
                        declared: PrecompileEncoding::Eip196Eip197,
                        required: vec![PrecompileEncoding::Eip2537],
                    },
                    PairingDisagreement::TargetGroupEncoding {
                        declared: TargetGroupEncodingIdentifier::Bn254V1,
                        required: TargetGroupEncodingIdentifier::Bls12381V1,
                    },
                ],
            }
        ))
    );
}
