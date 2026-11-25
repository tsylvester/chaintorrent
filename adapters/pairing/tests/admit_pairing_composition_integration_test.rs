#![cfg(feature = "mocks")]

use pairing::{
    AdmitPairingCompositionErrorReturn, AdmitPairingCompositionParamsOverrides,
    AdmitPairingCompositionSuccessReturnOverrides, LookupPairingDeclarationParamsOverrides,
    MockIPairingAdapterFailureMode, PAIRING_CONCRETES, PairingConcrete, PairingCurve,
    PairingDisagreement, PrecompileEncoding, TARGET_GROUP_ENCODING_IDENTIFIERS,
    TargetGroupEncodingIdentifier, VerifierGroupArithmetic, admit_pairing_composition,
    build_admit_pairing_composition_deps, build_admit_pairing_composition_params,
    build_admit_pairing_composition_payload, build_admit_pairing_composition_success_return,
    build_lookup_pairing_declaration_deps, build_lookup_pairing_declaration_params,
    build_lookup_pairing_declaration_payload, lookup_pairing_declaration,
};

/// Contract: entry `admission_compares_the_provisions_of_the_member_named`;
///   each member of `PAIRING_CONCRETES` with the required values read from the
///   declaration the public `lookup_pairing_declaration` returns for it → `Ok`
///   whose `identifier` is that member; and, for each member, the required
///   target-group encoding replaced by a value of
///   `TARGET_GROUP_ENCODING_IDENTIFIERS` the member does not declare →
///   `Err(Incompatible)` naming the member whose `disagreements` holds one
///   `PairingDisagreement::TargetGroupEncoding` carrying the member's declared
///   value and that required value.
/// Boundary: the public call `admit_pairing_composition`, the route extended
///   with the real `lookup_pairing_declaration` and the member's declaration
///   constant.
/// Mocked:   nothing; the route has no outer-edge collaborator, so the test
///   does not prove the mock member.
/// Arrange:  the block body written once and run over every member of
///   `PAIRING_CONCRETES`, the deps built with the public
///   `lookup_pairing_declaration`, the requirements built from the declaration
///   that function returns for the member, and, for the refusal, the
///   target-group override chosen from `TARGET_GROUP_ENCODING_IDENTIFIERS` as
///   a value unequal to the declared one, so an admission that reads one fixed
///   member's declaration, or that ignores the target-group requirement,
///   fails.
/// Act:      `admit_pairing_composition(&deps, params, payload)` per member and
///   per requirement set.
/// Assert:   the first result for each member equals
///   `Ok(build_admit_pairing_composition_success_return(...))` built with the
///   member as the identifier override; the second result equals
///   `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier,
///   disagreements })` with the member as the identifier and `disagreements`
///   holding a single `PairingDisagreement::TargetGroupEncoding { declared,
///   required }`, where `declared` is the target-group encoding of the
///   declaration the public lookup returned for the member and `required` is
///   the chosen value.
#[test]
fn admission_compares_the_provisions_of_the_member_named() {
    for member in PAIRING_CONCRETES {
        // Arrange
        let Ok(looked_up) = lookup_pairing_declaration(
            &build_lookup_pairing_declaration_deps(Default::default()),
            build_lookup_pairing_declaration_params(LookupPairingDeclarationParamsOverrides {
                concrete: Some(*member),
            }),
            build_lookup_pairing_declaration_payload(Default::default()),
        );
        let declaration = looked_up.declaration;
        let required_curve: PairingCurve = declaration.curve;
        let required_verifier_group_arithmetic: VerifierGroupArithmetic =
            declaration.verifier_group_arithmetic;
        let required_precompile_encodings: Vec<PrecompileEncoding> =
            vec![declaration.precompile_encoding];
        let required_target_group_encoding: TargetGroupEncodingIdentifier =
            declaration.target_group_encoding;

        let deps =
            build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default());
        let agreed =
            build_admit_pairing_composition_params(AdmitPairingCompositionParamsOverrides {
                identifier: Some(*member),
                required_curve: Some(required_curve),
                required_verifier_group_arithmetic: Some(required_verifier_group_arithmetic),
                required_precompile_encodings: Some(required_precompile_encodings.clone()),
                required_target_group_encoding: Some(required_target_group_encoding),
            });

        // Act
        let admitted = admit_pairing_composition(
            &deps,
            agreed,
            build_admit_pairing_composition_payload(Default::default()),
        );

        // Assert
        assert_eq!(
            admitted,
            Ok(build_admit_pairing_composition_success_return(
                AdmitPairingCompositionSuccessReturnOverrides {
                    identifier: Some(*member),
                }
            ))
        );

        // Arrange — a required target-group encoding the member does not declare
        for refused_target in TARGET_GROUP_ENCODING_IDENTIFIERS
            .iter()
            .copied()
            .filter(|identifier| *identifier != declaration.target_group_encoding)
        {
            let refused =
                build_admit_pairing_composition_params(AdmitPairingCompositionParamsOverrides {
                    identifier: Some(*member),
                    required_curve: Some(required_curve),
                    required_verifier_group_arithmetic: Some(required_verifier_group_arithmetic),
                    required_precompile_encodings: Some(required_precompile_encodings.clone()),
                    required_target_group_encoding: Some(refused_target),
                });

            // Act
            let result = admit_pairing_composition(
                &deps,
                refused,
                build_admit_pairing_composition_payload(Default::default()),
            );

            // Assert
            assert_eq!(
                result,
                Err(AdmitPairingCompositionErrorReturn::Incompatible {
                    identifier: *member,
                    disagreements: vec![PairingDisagreement::TargetGroupEncoding {
                        declared: declaration.target_group_encoding,
                        required: refused_target,
                    }],
                })
            );
        }
    }
}
// admission_compares_the_provisions_of_the_member_named

/// Contract: entry `admission_names_each_disagreeing_provision_for_each_declared_member`;
///   each member of `PAIRING_CONCRETES` with the required values read from the
///   declaration the public `lookup_pairing_declaration` returns for it, one
///   provision at a time replaced by the same provision of the declaration the
///   public lookup returns for a member of `PAIRING_CONCRETES` whose declared
///   curve differs → `Err(Incompatible)` naming the member whose
///   `disagreements` holds the one `PairingDisagreement` of the replaced
///   provision, carrying the member's declared value and the replaced required
///   value, the precompile provision carrying the required set holding the
///   other member's declared encoding.
/// Boundary: the public call `admit_pairing_composition`, the route extended
///   with the real `lookup_pairing_declaration` and the declaration constants
///   of the member and of the member of the other curve.
/// Mocked:   nothing; the route has no outer-edge collaborator, so the test
///   does not prove the mock member.
/// Arrange:  the block body written once and run over every member of
///   `PAIRING_CONCRETES` and, within each, over the curve, verifier-group
///   arithmetic, precompile encoding, and target-group encoding provisions,
///   the other-curve member chosen from `PAIRING_CONCRETES` by comparing the
///   declared curves the public lookup returns, so an admission that omits a
///   provision, names a provision that agrees, or names another provision
///   fails.
/// Act:      `admit_pairing_composition(&deps, params, payload)` per member and
///   per replaced provision.
/// Assert:   each result equals the whole
///   `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier,
///   disagreements })` with the member as the identifier and `disagreements`
///   holding the one `PairingDisagreement` of the replaced provision, its
///   `declared` read from the member's declaration and its `required` from the
///   other-curve member's declaration.
#[test]
fn admission_names_each_disagreeing_provision_for_each_declared_member() {
    for member in PAIRING_CONCRETES {
        // Arrange
        let Ok(member_looked_up) = lookup_pairing_declaration(
            &build_lookup_pairing_declaration_deps(Default::default()),
            build_lookup_pairing_declaration_params(LookupPairingDeclarationParamsOverrides {
                concrete: Some(*member),
            }),
            build_lookup_pairing_declaration_payload(Default::default()),
        );
        let declaration = member_looked_up.declaration;
        let mut other_declaration = None;
        for other in PAIRING_CONCRETES {
            let Ok(other_looked_up) = lookup_pairing_declaration(
                &build_lookup_pairing_declaration_deps(Default::default()),
                build_lookup_pairing_declaration_params(LookupPairingDeclarationParamsOverrides {
                    concrete: Some(*other),
                }),
                build_lookup_pairing_declaration_payload(Default::default()),
            );
            if other_looked_up.declaration.curve != declaration.curve {
                other_declaration = Some(other_looked_up.declaration);
            }
        }
        let Some(other_declaration) = other_declaration else {
            continue;
        };

        let deps =
            build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default());

        // Arrange — the curve provision replaced
        let params =
            build_admit_pairing_composition_params(AdmitPairingCompositionParamsOverrides {
                identifier: Some(*member),
                required_curve: Some(other_declaration.curve),
                required_verifier_group_arithmetic: Some(declaration.verifier_group_arithmetic),
                required_precompile_encodings: Some(vec![declaration.precompile_encoding]),
                required_target_group_encoding: Some(declaration.target_group_encoding),
            });

        // Act
        let result = admit_pairing_composition(
            &deps,
            params,
            build_admit_pairing_composition_payload(Default::default()),
        );

        // Assert
        assert_eq!(
            result,
            Err(AdmitPairingCompositionErrorReturn::Incompatible {
                identifier: *member,
                disagreements: vec![PairingDisagreement::Curve {
                    declared: declaration.curve,
                    required: other_declaration.curve,
                }],
            })
        );

        // Arrange — the verifier-group arithmetic provision replaced
        let params =
            build_admit_pairing_composition_params(AdmitPairingCompositionParamsOverrides {
                identifier: Some(*member),
                required_curve: Some(declaration.curve),
                required_verifier_group_arithmetic: Some(
                    other_declaration.verifier_group_arithmetic,
                ),
                required_precompile_encodings: Some(vec![declaration.precompile_encoding]),
                required_target_group_encoding: Some(declaration.target_group_encoding),
            });

        // Act
        let result = admit_pairing_composition(
            &deps,
            params,
            build_admit_pairing_composition_payload(Default::default()),
        );

        // Assert
        assert_eq!(
            result,
            Err(AdmitPairingCompositionErrorReturn::Incompatible {
                identifier: *member,
                disagreements: vec![PairingDisagreement::VerifierGroupArithmetic {
                    declared: declaration.verifier_group_arithmetic,
                    required: other_declaration.verifier_group_arithmetic,
                }],
            })
        );

        // Arrange — the precompile encoding provision replaced
        let params =
            build_admit_pairing_composition_params(AdmitPairingCompositionParamsOverrides {
                identifier: Some(*member),
                required_curve: Some(declaration.curve),
                required_verifier_group_arithmetic: Some(declaration.verifier_group_arithmetic),
                required_precompile_encodings: Some(vec![other_declaration.precompile_encoding]),
                required_target_group_encoding: Some(declaration.target_group_encoding),
            });

        // Act
        let result = admit_pairing_composition(
            &deps,
            params,
            build_admit_pairing_composition_payload(Default::default()),
        );

        // Assert
        assert_eq!(
            result,
            Err(AdmitPairingCompositionErrorReturn::Incompatible {
                identifier: *member,
                disagreements: vec![PairingDisagreement::PrecompileEncoding {
                    declared: declaration.precompile_encoding,
                    required: vec![other_declaration.precompile_encoding],
                }],
            })
        );

        // Arrange — the target-group encoding provision replaced
        let params =
            build_admit_pairing_composition_params(AdmitPairingCompositionParamsOverrides {
                identifier: Some(*member),
                required_curve: Some(declaration.curve),
                required_verifier_group_arithmetic: Some(declaration.verifier_group_arithmetic),
                required_precompile_encodings: Some(vec![declaration.precompile_encoding]),
                required_target_group_encoding: Some(other_declaration.target_group_encoding),
            });

        // Act
        let result = admit_pairing_composition(
            &deps,
            params,
            build_admit_pairing_composition_payload(Default::default()),
        );

        // Assert
        assert_eq!(
            result,
            Err(AdmitPairingCompositionErrorReturn::Incompatible {
                identifier: *member,
                disagreements: vec![PairingDisagreement::TargetGroupEncoding {
                    declared: declaration.target_group_encoding,
                    required: other_declaration.target_group_encoding,
                }],
            })
        );
    }
}
// admission_names_each_disagreeing_provision_for_each_declared_member

/// Contract: entry `admission_names_every_disagreeing_provision_in_order_for_each_declared_member`;
///   each member of `PAIRING_CONCRETES` with every required value taken from
///   the declaration the public `lookup_pairing_declaration` returns for a
///   member of `PAIRING_CONCRETES` whose declared curve differs →
///   `Err(Incompatible)` naming the member whose `disagreements` holds the
///   curve, verifier-group arithmetic, precompile encoding, and target-group
///   encoding disagreement, in that order, each carrying the member's declared
///   value and the other member's value.
/// Boundary: the route of the previous block.
/// Mocked:   nothing; the route has no outer-edge collaborator, so the test
///   does not prove the mock member.
/// Arrange:  the block body written once and run over every member of
///   `PAIRING_CONCRETES`, the requirements taken wholly from the other-curve
///   member's declaration, so an admission that stops at the first
///   disagreement, or names the disagreements in another order, fails.
/// Act:      `admit_pairing_composition(&deps, params, payload)` per member.
/// Assert:   each result equals the whole
///   `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier,
///   disagreements })` with the member as the identifier and `disagreements`
///   holding every disagreement in order, each `declared` read from the
///   member's declaration and each `required` from the other-curve member's
///   declaration.
#[test]
fn admission_names_every_disagreeing_provision_in_order_for_each_declared_member() {
    for member in PAIRING_CONCRETES {
        // Arrange
        let Ok(member_looked_up) = lookup_pairing_declaration(
            &build_lookup_pairing_declaration_deps(Default::default()),
            build_lookup_pairing_declaration_params(LookupPairingDeclarationParamsOverrides {
                concrete: Some(*member),
            }),
            build_lookup_pairing_declaration_payload(Default::default()),
        );
        let declaration = member_looked_up.declaration;
        let mut other_declaration = None;
        for other in PAIRING_CONCRETES {
            let Ok(other_looked_up) = lookup_pairing_declaration(
                &build_lookup_pairing_declaration_deps(Default::default()),
                build_lookup_pairing_declaration_params(LookupPairingDeclarationParamsOverrides {
                    concrete: Some(*other),
                }),
                build_lookup_pairing_declaration_payload(Default::default()),
            );
            if other_looked_up.declaration.curve != declaration.curve {
                other_declaration = Some(other_looked_up.declaration);
            }
        }
        let Some(other_declaration) = other_declaration else {
            continue;
        };

        let deps =
            build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default());
        let params =
            build_admit_pairing_composition_params(AdmitPairingCompositionParamsOverrides {
                identifier: Some(*member),
                required_curve: Some(other_declaration.curve),
                required_verifier_group_arithmetic: Some(
                    other_declaration.verifier_group_arithmetic,
                ),
                required_precompile_encodings: Some(vec![other_declaration.precompile_encoding]),
                required_target_group_encoding: Some(other_declaration.target_group_encoding),
            });

        // Act
        let result = admit_pairing_composition(
            &deps,
            params,
            build_admit_pairing_composition_payload(Default::default()),
        );

        // Assert
        assert_eq!(
            result,
            Err(AdmitPairingCompositionErrorReturn::Incompatible {
                identifier: *member,
                disagreements: vec![
                    PairingDisagreement::Curve {
                        declared: declaration.curve,
                        required: other_declaration.curve,
                    },
                    PairingDisagreement::VerifierGroupArithmetic {
                        declared: declaration.verifier_group_arithmetic,
                        required: other_declaration.verifier_group_arithmetic,
                    },
                    PairingDisagreement::PrecompileEncoding {
                        declared: declaration.precompile_encoding,
                        required: vec![other_declaration.precompile_encoding],
                    },
                    PairingDisagreement::TargetGroupEncoding {
                        declared: declaration.target_group_encoding,
                        required: other_declaration.target_group_encoding,
                    },
                ],
            })
        );
    }
}
// admission_names_every_disagreeing_provision_in_order_for_each_declared_member

/// Contract: entry `mock_admission_compares_the_mock_declaration`;
///   `PairingConcrete::Mock(mode)` with the Bn254 required values → `Ok` whose
///   `identifier` is `PairingConcrete::Mock(mode)`, and with a required curve
///   of `PairingCurve::Bls12381` and the other Bn254 values →
///   `Err(Incompatible)` naming `PairingConcrete::Mock(mode)` whose
///   `disagreements` holds a single `PairingDisagreement::Curve` with the
///   declared `PairingCurve::Bn254`.
/// Boundary: the public call `admit_pairing_composition`, the route extended
///   with the real `lookup_pairing_declaration` and
///   `MOCK_I_PAIRING_ADAPTER_DECLARATION`.
/// Mocked:   the mock member itself, so the test proves the identifier is
///   carried and the mock declaration is compared, and no real member's.
/// Arrange:  the identifier override
///   `PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve)`,
///   a mode other than `Succeeds`, called once with the Bn254 required values
///   and once with the required curve `PairingCurve::Bls12381`.
/// Act:      `admit_pairing_composition(&deps, params, payload)` per
///   requirement set.
/// Assert:   the first result equals
///   `Ok(build_admit_pairing_composition_success_return(...))` with the mock
///   identifier as the identifier override, and the second equals
///   `Err(AdmitPairingCompositionErrorReturn::Incompatible { identifier:
///   PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve),
///   disagreements: vec![PairingDisagreement::Curve { declared:
///   PairingCurve::Bn254, required: PairingCurve::Bls12381 }] })`.
#[cfg(feature = "mocks")]
#[test]
fn mock_admission_compares_the_mock_declaration() {
    // Arrange
    let deps = build_admit_pairing_composition_deps(lookup_pairing_declaration, Default::default());
    let admitted = build_admit_pairing_composition_params(AdmitPairingCompositionParamsOverrides {
        identifier: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::DecodeG1NotOnCurve,
        )),
        required_curve: Some(PairingCurve::Bn254),
        required_verifier_group_arithmetic: Some(VerifierGroupArithmetic::FirstGroupOnly),
        required_precompile_encodings: Some(vec![PrecompileEncoding::Eip196Eip197]),
        required_target_group_encoding: Some(TargetGroupEncodingIdentifier::Bn254V1),
    });

    // Act
    let result = admit_pairing_composition(
        &deps,
        admitted,
        build_admit_pairing_composition_payload(Default::default()),
    );

    // Assert
    assert_eq!(
        result,
        Ok(build_admit_pairing_composition_success_return(
            AdmitPairingCompositionSuccessReturnOverrides {
                identifier: Some(PairingConcrete::Mock(
                    MockIPairingAdapterFailureMode::DecodeG1NotOnCurve,
                )),
            }
        ))
    );

    // Arrange — the required curve of the other curve
    let refused = build_admit_pairing_composition_params(AdmitPairingCompositionParamsOverrides {
        identifier: Some(PairingConcrete::Mock(
            MockIPairingAdapterFailureMode::DecodeG1NotOnCurve,
        )),
        required_curve: Some(PairingCurve::Bls12381),
        required_verifier_group_arithmetic: Some(VerifierGroupArithmetic::FirstGroupOnly),
        required_precompile_encodings: Some(vec![PrecompileEncoding::Eip196Eip197]),
        required_target_group_encoding: Some(TargetGroupEncodingIdentifier::Bn254V1),
    });

    // Act
    let refused_result = admit_pairing_composition(
        &deps,
        refused,
        build_admit_pairing_composition_payload(Default::default()),
    );

    // Assert
    assert_eq!(
        refused_result,
        Err(AdmitPairingCompositionErrorReturn::Incompatible {
            identifier: PairingConcrete::Mock(MockIPairingAdapterFailureMode::DecodeG1NotOnCurve,),
            disagreements: vec![PairingDisagreement::Curve {
                declared: PairingCurve::Bn254,
                required: PairingCurve::Bls12381,
            }],
        })
    );
}
// mock_admission_compares_the_mock_declaration

// Public-surface enumeration: the own entries
//   `admission_compares_the_provisions_of_the_member_named`,
//   proven by `admission_compares_the_provisions_of_the_member_named`,
//   `admission_names_each_disagreeing_provision_for_each_declared_member`,
//   proven by `admission_names_each_disagreeing_provision_for_each_declared_member`,
//   `admission_names_every_disagreeing_provision_in_order_for_each_declared_member`,
//   proven by `admission_names_every_disagreeing_provision_in_order_for_each_declared_member`,
//   and `mock_admission_compares_the_mock_declaration`,
//   proven by `mock_admission_compares_the_mock_declaration`.
