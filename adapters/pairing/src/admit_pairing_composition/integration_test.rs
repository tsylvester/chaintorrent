use super::provides::{
    AdmitPairingCompositionErrorReturn, AdmitPairingCompositionParamsOverrides,
    AdmitPairingCompositionSuccessReturnOverrides, PairingDisagreement, admit_pairing_composition,
    build_admit_pairing_composition_deps, build_admit_pairing_composition_params,
    build_admit_pairing_composition_payload, build_admit_pairing_composition_success_return,
};
use crate::factory::provides::{
    MockIPairingAdapterFailureMode, PairingConcrete, PairingCurve, PrecompileEncoding,
    TargetGroupEncodingIdentifier, VerifierGroupArithmetic,
};
use crate::lookup_pairing_declaration::provides::lookup_pairing_declaration;

/// Contract: entry `admission_compares_the_provisions_of_the_member_named`;
///   `PairingConcrete::Bn254Halo2curves` with the required values
///   `PairingCurve::Bn254`, `VerifierGroupArithmetic::FirstGroupOnly`, the set
///   `vec![PrecompileEncoding::Eip196Eip197]`, and
///   `TargetGroupEncodingIdentifier::Bn254V1` → `Ok` whose `identifier` is
///   `PairingConcrete::Bn254Halo2curves`.
/// Boundary: `admit_pairing_composition` then the real
///   `lookup_pairing_declaration` then the declaration constant of the member
///   named.
/// Mocked:   nothing; the route has no outer-edge collaborator.
/// Arrange:  `build_admit_pairing_composition_deps` handed the real
///   `lookup_pairing_declaration`; the identifier override
///   `PairingConcrete::Bn254Halo2curves` and the required values stated as
///   independent literals in the contract, a member whose provisions are the
///   Bn254 values, so an admission that reads another curve's declaration
///   fails.
/// Act:      `admit_pairing_composition(&deps, params, payload)`.
/// Assert:   the result equals `Ok(build_admit_pairing_composition_success_return(
///   AdmitPairingCompositionSuccessReturnOverrides { identifier:
///   Some(PairingConcrete::Bn254Halo2curves) }))`.
#[test]
fn a_bn254_member_is_admitted_against_bn254_requirements() {
    // Arrange
    let deps = build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default());
    let params = build_admit_pairing_composition_params(AdmitPairingCompositionParamsOverrides {
        identifier: Some(PairingConcrete::Bn254Halo2curves),
        required_curve: Some(PairingCurve::Bn254),
        required_verifier_group_arithmetic: Some(VerifierGroupArithmetic::FirstGroupOnly),
        required_precompile_encodings: Some(vec![PrecompileEncoding::Eip196Eip197]),
        required_target_group_encoding: Some(TargetGroupEncodingIdentifier::Bn254V1),
    });
    let payload = build_admit_pairing_composition_payload(Default::default());

    // Act
    let result = admit_pairing_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_pairing_composition_success_return(
            AdmitPairingCompositionSuccessReturnOverrides {
                identifier: Some(PairingConcrete::Bn254Halo2curves),
                ..Default::default()
            }
        ))
    );
}
// a_bn254_member_is_admitted_against_bn254_requirements

/// Contract: entry `admission_compares_the_provisions_of_the_member_named`;
///   `PairingConcrete::Bls12381Arkworks` with the required values
///   `PairingCurve::Bls12381`, `VerifierGroupArithmetic::BothGroups`, the set
///   `vec![PrecompileEncoding::Eip2537]`, and
///   `TargetGroupEncodingIdentifier::Bls12381V1` → `Ok` whose `identifier` is
///   `PairingConcrete::Bls12381Arkworks`.
/// Boundary: the route of the previous block.
/// Mocked:   nothing; the route has no outer-edge collaborator.
/// Arrange:  the real lookup in the deps; the identifier override
///   `PairingConcrete::Bls12381Arkworks` and the required values stated as
///   independent literals in the contract, a member of the other curve, so an
///   admission that reads a Bn254 declaration fails.
/// Act:      `admit_pairing_composition(&deps, params, payload)`.
/// Assert:   the result equals `Ok(build_admit_pairing_composition_success_return(
///   AdmitPairingCompositionSuccessReturnOverrides { identifier:
///   Some(PairingConcrete::Bls12381Arkworks) }))`.
#[test]
fn a_bls12_381_member_is_admitted_against_bls12_381_requirements() {
    // Arrange
    let deps = build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default());
    let params = build_admit_pairing_composition_params(AdmitPairingCompositionParamsOverrides {
        identifier: Some(PairingConcrete::Bls12381Arkworks),
        required_curve: Some(PairingCurve::Bls12381),
        required_verifier_group_arithmetic: Some(VerifierGroupArithmetic::BothGroups),
        required_precompile_encodings: Some(vec![PrecompileEncoding::Eip2537]),
        required_target_group_encoding: Some(TargetGroupEncodingIdentifier::Bls12381V1),
    });
    let payload = build_admit_pairing_composition_payload(Default::default());

    // Act
    let result = admit_pairing_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_pairing_composition_success_return(
            AdmitPairingCompositionSuccessReturnOverrides {
                identifier: Some(PairingConcrete::Bls12381Arkworks),
                ..Default::default()
            }
        ))
    );
}
// a_bls12_381_member_is_admitted_against_bls12_381_requirements

/// Contract: entry `admission_compares_the_provisions_of_the_member_named`;
///   `PairingConcrete::Bn254Arkworks` with the Bls12-381 required values of the
///   previous block → `Err(Incompatible)` naming `PairingConcrete::Bn254Arkworks`
///   whose `disagreements` holds the curve, verifier-group arithmetic,
///   precompile encoding, and target-group encoding disagreement with the
///   declared values `PairingCurve::Bn254`,
///   `VerifierGroupArithmetic::FirstGroupOnly`,
///   `PrecompileEncoding::Eip196Eip197`, and
///   `TargetGroupEncodingIdentifier::Bn254V1`.
/// Boundary: the route of the previous blocks.
/// Mocked:   nothing; the route has no outer-edge collaborator.
/// Arrange:  the real lookup in the deps; the identifier override
///   `PairingConcrete::Bn254Arkworks` and the Bls12-381 required values as
///   independent literals, a member whose every provision differs from them, so
///   an admission that reads another curve's declaration or compares fewer
///   provisions fails.
/// Act:      `admit_pairing_composition(&deps, params, payload)`.
/// Assert:   the result equals the whole
///   `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier:
///   PairingConcrete::Bn254Arkworks, disagreements })` with `disagreements`
///   holding every disagreement in order, each stated with its declared and
///   required values as literals.
#[test]
fn a_bn254_member_is_refused_against_bls12_381_requirements() {
    // Arrange
    let deps = build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default());
    let params = build_admit_pairing_composition_params(AdmitPairingCompositionParamsOverrides {
        identifier: Some(PairingConcrete::Bn254Arkworks),
        required_curve: Some(PairingCurve::Bls12381),
        required_verifier_group_arithmetic: Some(VerifierGroupArithmetic::BothGroups),
        required_precompile_encodings: Some(vec![PrecompileEncoding::Eip2537]),
        required_target_group_encoding: Some(TargetGroupEncodingIdentifier::Bls12381V1),
    });
    let payload = build_admit_pairing_composition_payload(Default::default());

    // Act
    let result = admit_pairing_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitPairingCompositionErrorReturn::Incompatible {
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
        })
    );
}
// a_bn254_member_is_refused_against_bls12_381_requirements

/// Contract: entry `admission_compares_the_provisions_of_the_member_named`;
///   `PairingConcrete::Bls12381Halo2curves` with the Bn254 required values of
///   `a_bn254_member_is_admitted_against_bn254_requirements` →
///   `Err(Incompatible)` naming `PairingConcrete::Bls12381Halo2curves` whose
///   `disagreements` holds every disagreement with the declared values
///   `PairingCurve::Bls12381`, `VerifierGroupArithmetic::BothGroups`,
///   `PrecompileEncoding::Eip2537`, and
///   `TargetGroupEncodingIdentifier::Bls12381V1`.
/// Boundary: the route of the previous blocks.
/// Mocked:   nothing; the route has no outer-edge collaborator.
/// Arrange:  the real lookup in the deps; the identifier override
///   `PairingConcrete::Bls12381Halo2curves` and the Bn254 required values as
///   independent literals, a member of the other curve from the previous block,
///   so an admission that reads a fixed member's declaration fails a refusal
///   block.
/// Act:      `admit_pairing_composition(&deps, params, payload)`.
/// Assert:   the result equals the whole
///   `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier:
///   PairingConcrete::Bls12381Halo2curves, disagreements })` with `disagreements`
///   holding every disagreement in order, each stated with its declared and
///   required values as literals.
#[test]
fn a_bls12_381_member_is_refused_against_bn254_requirements() {
    // Arrange
    let deps = build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default());
    let params = build_admit_pairing_composition_params(AdmitPairingCompositionParamsOverrides {
        identifier: Some(PairingConcrete::Bls12381Halo2curves),
        required_curve: Some(PairingCurve::Bn254),
        required_verifier_group_arithmetic: Some(VerifierGroupArithmetic::FirstGroupOnly),
        required_precompile_encodings: Some(vec![PrecompileEncoding::Eip196Eip197]),
        required_target_group_encoding: Some(TargetGroupEncodingIdentifier::Bn254V1),
    });
    let payload = build_admit_pairing_composition_payload(Default::default());

    // Act
    let result = admit_pairing_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitPairingCompositionErrorReturn::Incompatible {
            identifier: PairingConcrete::Bls12381Halo2curves,
            disagreements: vec![
                PairingDisagreement::Curve {
                    declared: PairingCurve::Bls12381,
                    required: PairingCurve::Bn254,
                },
                PairingDisagreement::VerifierGroupArithmetic {
                    declared: VerifierGroupArithmetic::BothGroups,
                    required: VerifierGroupArithmetic::FirstGroupOnly,
                },
                PairingDisagreement::PrecompileEncoding {
                    declared: PrecompileEncoding::Eip2537,
                    required: vec![PrecompileEncoding::Eip196Eip197],
                },
                PairingDisagreement::TargetGroupEncoding {
                    declared: TargetGroupEncodingIdentifier::Bls12381V1,
                    required: TargetGroupEncodingIdentifier::Bn254V1,
                },
            ],
        })
    );
}
// a_bls12_381_member_is_refused_against_bn254_requirements

/// Contract: entry `mock_admission_compares_the_mock_declaration`;
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)`
///   with the Bn254 required values of
///   `a_bn254_member_is_admitted_against_bn254_requirements` → `Ok` whose
///   `identifier` is that mock identifier.
/// Boundary: `admit_pairing_composition` then the real
///   `lookup_pairing_declaration` then `MOCK_I_PAIRING_ADAPTER_DECLARATION`.
/// Mocked:   the mock member itself, so the test proves the mock declaration is
///   read and no real member's.
/// Arrange:  the real lookup in the deps; the identifier override
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)`,
///   a mode other than `Succeeds`, and the Bn254 required values as independent
///   literals.
/// Act:      `admit_pairing_composition(&deps, params, payload)`.
/// Assert:   the result equals `Ok(build_admit_pairing_composition_success_return(
///   AdmitPairingCompositionSuccessReturnOverrides { identifier:
///   Some(PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve))
///   }))`.
#[test]
fn the_mock_member_is_admitted_against_the_provisions_its_declaration_states() {
    // Arrange
    let deps = build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default());
    let params = build_admit_pairing_composition_params(AdmitPairingCompositionParamsOverrides {
        identifier: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::DecodeG1NotOnCurve,
        )),
        required_curve: Some(PairingCurve::Bn254),
        required_verifier_group_arithmetic: Some(VerifierGroupArithmetic::FirstGroupOnly),
        required_precompile_encodings: Some(vec![PrecompileEncoding::Eip196Eip197]),
        required_target_group_encoding: Some(TargetGroupEncodingIdentifier::Bn254V1),
    });
    let payload = build_admit_pairing_composition_payload(Default::default());

    // Act
    let result = admit_pairing_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_pairing_composition_success_return(
            AdmitPairingCompositionSuccessReturnOverrides {
                identifier: Some(PairingConcrete::Mock(
                    MockIPairingAdapterFailureMode::DecodeG1NotOnCurve,
                )),
                ..Default::default()
            }
        ))
    );
}
// the_mock_member_is_admitted_against_the_provisions_its_declaration_states

/// Contract: entry `mock_admission_compares_the_mock_declaration`;
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG2NotOnCurve)`
///   with the Bls12-381 required values of
///   `a_bn254_member_is_refused_against_bls12_381_requirements` →
///   `Err(Incompatible)` naming that mock identifier whose `disagreements`
///   holds every disagreement with the declared Bn254 values the mock
///   declaration states.
/// Boundary: the route of the previous block.
/// Mocked:   the mock member itself, so the test proves the identifier is
///   carried into the refusal.
/// Arrange:  the real lookup in the deps; the identifier override
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG2NotOnCurve)`,
///   a mode that differs from the previous block's, and the Bls12-381 required
///   values as independent literals, so a refusal that names another mode or a
///   real member fails.
/// Act:      `admit_pairing_composition(&deps, params, payload)`.
/// Assert:   the result equals the whole
///   `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier:
///   PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG2NotOnCurve),
///   disagreements })` with `disagreements` holding every disagreement in order,
///   each stated with its declared and required values as literals.
#[test]
fn the_mock_member_is_refused_naming_its_identifier() {
    // Arrange
    let deps = build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default());
    let params = build_admit_pairing_composition_params(AdmitPairingCompositionParamsOverrides {
        identifier: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::DecodeG2NotOnCurve,
        )),
        required_curve: Some(PairingCurve::Bls12381),
        required_verifier_group_arithmetic: Some(VerifierGroupArithmetic::BothGroups),
        required_precompile_encodings: Some(vec![PrecompileEncoding::Eip2537]),
        required_target_group_encoding: Some(TargetGroupEncodingIdentifier::Bls12381V1),
    });
    let payload = build_admit_pairing_composition_payload(Default::default());

    // Act
    let result = admit_pairing_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitPairingCompositionErrorReturn::Incompatible {
            identifier: PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG2NotOnCurve,),
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
        })
    );
}
// the_mock_member_is_refused_naming_its_identifier

// Private-surface enumeration: the own entries
//   `admission_compares_the_provisions_of_the_member_named`,
//   proven by `a_bn254_member_is_admitted_against_bn254_requirements`,
//   `a_bls12_381_member_is_admitted_against_bls12_381_requirements`,
//   `a_bn254_member_is_refused_against_bls12_381_requirements`, and
//   `a_bls12_381_member_is_refused_against_bn254_requirements`,
//   and `mock_admission_compares_the_mock_declaration`,
//   proven by `the_mock_member_is_admitted_against_the_provisions_its_declaration_states`
//   and `the_mock_member_is_refused_naming_its_identifier`.
