use super::provides::{
    AdmitPairingCompositionErrorReturn, AdmitPairingCompositionParamsOverrides,
    AdmitPairingCompositionSuccessReturnOverrides, PairingDisagreement, admit_pairing_composition,
    build_admit_pairing_composition_deps, build_admit_pairing_composition_params,
    build_admit_pairing_composition_payload, build_admit_pairing_composition_success_return,
};
use crate::factory::provides::{
    PairingConcrete, PairingCurve, PrecompileEncoding, TargetGroupEncodingIdentifier,
    VerifierGroupArithmetic,
};
use crate::lookup_pairing_declaration::provides::mock_lookup_pairing_declaration;

/// Contract: the declaration the lookup returns provides `PairingCurve::Bn254`,
///   `VerifierGroupArithmetic::FirstGroupOnly`, `PrecompileEncoding::Eip196Eip197`,
///   and `TargetGroupEncodingIdentifier::Bn254V1`, and the params require those
///   values with a precompile set holding that encoding → `Ok` whose `identifier`
///   is `params.identifier`.
/// Collaborators: `mock_lookup_pairing_declaration`, the official function mock of
///   the direct dependency, returning the declaration built at the defaults of
///   `build_pairing_declaration`; fixtures `build_admit_pairing_composition_deps`,
///   `build_admit_pairing_composition_params`,
///   `build_admit_pairing_composition_payload`, and
///   `build_admit_pairing_composition_success_return`.
/// Arrange: `build_admit_pairing_composition_deps` handed
///   `mock_lookup_pairing_declaration`; `build_admit_pairing_composition_params`
///   with the identifier override `PairingConcrete::Bls12381Arkworks`, an identifier
///   that differs from the one the mock declaration carries, so an admission that
///   returns the declaration's `concrete` fails, and the precompile override
///   `vec![PrecompileEncoding::Eip2537, PrecompileEncoding::Eip196Eip197]`, a set
///   that holds the declared encoding beside another, so an admission that requires
///   the set to equal the declared encoding fails.
/// Act:     `admit_pairing_composition(&deps, params, payload)`.
/// Assert:  the result equals `Ok(build_admit_pairing_composition_success_return(
///   AdmitPairingCompositionSuccessReturnOverrides { identifier:
///   Some(PairingConcrete::Bls12381Arkworks) }))`.
#[test]
fn admit_returns_the_identifier_when_every_provision_agrees() {
    // Arrange
    let deps =
        build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, Default::default());
    let params = build_admit_pairing_composition_params(AdmitPairingCompositionParamsOverrides {
        identifier: Some(PairingConcrete::Bls12381Arkworks),
        required_precompile_encodings: Some(vec![
            PrecompileEncoding::Eip2537,
            PrecompileEncoding::Eip196Eip197,
        ]),
        ..Default::default()
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
            }
        ))
    );
}
// admit_returns_the_identifier_when_every_provision_agrees

/// Contract: the declared curve is `PairingCurve::Bn254` and the params require
///   `PairingCurve::Bls12381`, every other provision agreeing →
///   `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier,
///   disagreements })` whose `disagreements` holds `PairingDisagreement::Curve
///   { declared: PairingCurve::Bn254, required: PairingCurve::Bls12381 }` alone.
/// Collaborators: the fixtures and `mock_lookup_pairing_declaration` of the
///   previous block.
/// Arrange: the identifier override `PairingConcrete::Bls12381Halo2curves`, which
///   the mock declaration does not carry, so a refusal that names the
///   declaration's member fails, and the required-curve override
///   `PairingCurve::Bls12381`, a value that differs from the declared one in this
///   provision alone, so an admission that compares more or fewer provisions
///   fails.
/// Act:     `admit_pairing_composition(&deps, params, payload)`.
/// Assert:  the result equals `Err(AdmitPairingCompositionErrorReturn::Incompatible
///   { identifier: PairingConcrete::Bls12381Halo2curves, disagreements:
///   vec![PairingDisagreement::Curve { declared: PairingCurve::Bn254, required:
///   PairingCurve::Bls12381 }] })`.
#[test]
fn admit_names_a_curve_disagreement() {
    // Arrange
    let deps =
        build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, Default::default());
    let params = build_admit_pairing_composition_params(AdmitPairingCompositionParamsOverrides {
        identifier: Some(PairingConcrete::Bls12381Halo2curves),
        required_curve: Some(PairingCurve::Bls12381),
        ..Default::default()
    });
    let payload = build_admit_pairing_composition_payload(Default::default());

    // Act
    let result = admit_pairing_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitPairingCompositionErrorReturn::Incompatible {
            identifier: PairingConcrete::Bls12381Halo2curves,
            disagreements: vec![PairingDisagreement::Curve {
                declared: PairingCurve::Bn254,
                required: PairingCurve::Bls12381,
            }],
        })
    );
}
// admit_names_a_curve_disagreement

/// Contract: the declared arithmetic is `VerifierGroupArithmetic::FirstGroupOnly`
///   and the params require `VerifierGroupArithmetic::BothGroups`, every other
///   provision agreeing → `Err(Incompatible)` whose `disagreements` holds
///   `PairingDisagreement::VerifierGroupArithmetic { declared:
///   VerifierGroupArithmetic::FirstGroupOnly, required:
///   VerifierGroupArithmetic::BothGroups }` alone.
/// Collaborators: the fixtures and `mock_lookup_pairing_declaration` of the
///   previous blocks.
/// Arrange: the identifier override `PairingConcrete::Bn254Halo2curves` and the
///   required-arithmetic override `VerifierGroupArithmetic::BothGroups`, a value
///   that differs from the declared one in this provision alone.
/// Act:     `admit_pairing_composition(&deps, params, payload)`.
/// Assert:  the result equals `Err(AdmitPairingCompositionErrorReturn::Incompatible
///   { identifier: PairingConcrete::Bn254Halo2curves, disagreements:
///   vec![PairingDisagreement::VerifierGroupArithmetic { declared:
///   VerifierGroupArithmetic::FirstGroupOnly, required:
///   VerifierGroupArithmetic::BothGroups }] })`.
#[test]
fn admit_names_a_verifier_group_arithmetic_disagreement() {
    // Arrange
    let deps =
        build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, Default::default());
    let params = build_admit_pairing_composition_params(AdmitPairingCompositionParamsOverrides {
        identifier: Some(PairingConcrete::Bn254Halo2curves),
        required_verifier_group_arithmetic: Some(VerifierGroupArithmetic::BothGroups),
        ..Default::default()
    });
    let payload = build_admit_pairing_composition_payload(Default::default());

    // Act
    let result = admit_pairing_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitPairingCompositionErrorReturn::Incompatible {
            identifier: PairingConcrete::Bn254Halo2curves,
            disagreements: vec![PairingDisagreement::VerifierGroupArithmetic {
                declared: VerifierGroupArithmetic::FirstGroupOnly,
                required: VerifierGroupArithmetic::BothGroups,
            }],
        })
    );
}
// admit_names_a_verifier_group_arithmetic_disagreement

/// Contract: the declared encoding is `PrecompileEncoding::Eip196Eip197` and the
///   required set is `vec![PrecompileEncoding::Eip2537]`, every other provision
///   agreeing → `Err(Incompatible)` whose `disagreements` holds
///   `PairingDisagreement::PrecompileEncoding { declared:
///   PrecompileEncoding::Eip196Eip197, required:
///   vec![PrecompileEncoding::Eip2537] }` alone.
/// Collaborators: the fixtures and `mock_lookup_pairing_declaration` of the
///   previous blocks.
/// Arrange: the identifier override `PairingConcrete::Bn254Arkworks` and the
///   precompile override `vec![PrecompileEncoding::Eip2537]`, a set that lacks
///   the declared encoding, so an admission that accepts any non-empty set
///   fails.
/// Act:     `admit_pairing_composition(&deps, params, payload)`.
/// Assert:  the result equals `Err(AdmitPairingCompositionErrorReturn::Incompatible
///   { identifier: PairingConcrete::Bn254Arkworks, disagreements:
///   vec![PairingDisagreement::PrecompileEncoding { declared:
///   PrecompileEncoding::Eip196Eip197, required:
///   vec![PrecompileEncoding::Eip2537] }] })`.
#[test]
fn admit_names_a_precompile_encoding_disagreement() {
    // Arrange
    let deps =
        build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, Default::default());
    let params = build_admit_pairing_composition_params(AdmitPairingCompositionParamsOverrides {
        identifier: Some(PairingConcrete::Bn254Arkworks),
        required_precompile_encodings: Some(vec![PrecompileEncoding::Eip2537]),
        ..Default::default()
    });
    let payload = build_admit_pairing_composition_payload(Default::default());

    // Act
    let result = admit_pairing_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitPairingCompositionErrorReturn::Incompatible {
            identifier: PairingConcrete::Bn254Arkworks,
            disagreements: vec![PairingDisagreement::PrecompileEncoding {
                declared: PrecompileEncoding::Eip196Eip197,
                required: vec![PrecompileEncoding::Eip2537],
            }],
        })
    );
}
// admit_names_a_precompile_encoding_disagreement

/// Contract: the declared target-group encoding is
///   `TargetGroupEncodingIdentifier::Bn254V1` and the params require
///   `TargetGroupEncodingIdentifier::Bls12381V1`, every other provision agreeing
///   → `Err(Incompatible)` whose `disagreements` holds
///   `PairingDisagreement::TargetGroupEncoding { declared:
///   TargetGroupEncodingIdentifier::Bn254V1, required:
///   TargetGroupEncodingIdentifier::Bls12381V1 }` alone.
/// Collaborators: the fixtures and `mock_lookup_pairing_declaration` of the
///   previous blocks.
/// Arrange: the identifier override `PairingConcrete::Bls12381Arkworks` and the
///   required-target-group override `TargetGroupEncodingIdentifier::Bls12381V1`,
///   a value that differs from the declared one in this provision alone.
/// Act:     `admit_pairing_composition(&deps, params, payload)`.
/// Assert:  the result equals `Err(AdmitPairingCompositionErrorReturn::Incompatible
///   { identifier: PairingConcrete::Bls12381Arkworks, disagreements:
///   vec![PairingDisagreement::TargetGroupEncoding { declared:
///   TargetGroupEncodingIdentifier::Bn254V1, required:
///   TargetGroupEncodingIdentifier::Bls12381V1 }] })`.
#[test]
fn admit_names_a_target_group_encoding_disagreement() {
    // Arrange
    let deps =
        build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, Default::default());
    let params = build_admit_pairing_composition_params(AdmitPairingCompositionParamsOverrides {
        identifier: Some(PairingConcrete::Bls12381Arkworks),
        required_target_group_encoding: Some(TargetGroupEncodingIdentifier::Bls12381V1),
        ..Default::default()
    });
    let payload = build_admit_pairing_composition_payload(Default::default());

    // Act
    let result = admit_pairing_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitPairingCompositionErrorReturn::Incompatible {
            identifier: PairingConcrete::Bls12381Arkworks,
            disagreements: vec![PairingDisagreement::TargetGroupEncoding {
                declared: TargetGroupEncodingIdentifier::Bn254V1,
                required: TargetGroupEncodingIdentifier::Bls12381V1,
            }],
        })
    );
}
// admit_names_a_target_group_encoding_disagreement

/// Contract: every provision disagrees, the params requiring
///   `PairingCurve::Bls12381`, `VerifierGroupArithmetic::BothGroups`, the set
///   `vec![PrecompileEncoding::Eip2537]`, and
///   `TargetGroupEncodingIdentifier::Bls12381V1` against the declared Bn254
///   values → `Err(Incompatible)` whose `disagreements` holds the curve, the
///   verifier-group arithmetic, the precompile encoding, and the target-group
///   encoding disagreement, in that order.
/// Collaborators: the fixtures and `mock_lookup_pairing_declaration` of the
///   previous blocks.
/// Arrange: the identifier override `PairingConcrete::Bn254Arkworks` and an
///   override of each required value to the Bls12-381 value named in the
///   contract, so an admission that stops at the first disagreement, or names
///   the disagreements in another order, fails.
/// Act:     `admit_pairing_composition(&deps, params, payload)`.
/// Assert:  the result equals `Err(AdmitPairingCompositionErrorReturn::Incompatible
///   { identifier: PairingConcrete::Bn254Arkworks, disagreements:
///   vec![PairingDisagreement::Curve { declared: PairingCurve::Bn254, required:
///   PairingCurve::Bls12381 }, PairingDisagreement::VerifierGroupArithmetic
///   { declared: VerifierGroupArithmetic::FirstGroupOnly, required:
///   VerifierGroupArithmetic::BothGroups },
///   PairingDisagreement::PrecompileEncoding { declared:
///   PrecompileEncoding::Eip196Eip197, required:
///   vec![PrecompileEncoding::Eip2537] },
///   PairingDisagreement::TargetGroupEncoding { declared:
///   TargetGroupEncodingIdentifier::Bn254V1, required:
///   TargetGroupEncodingIdentifier::Bls12381V1 }] })`.
#[test]
fn admit_names_every_disagreeing_provision_in_order() {
    // Arrange
    let deps =
        build_admit_pairing_composition_deps(mock_lookup_pairing_declaration, Default::default());
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
// admit_names_every_disagreeing_provision_in_order
