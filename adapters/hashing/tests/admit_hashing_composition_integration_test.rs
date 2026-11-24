#![cfg(feature = "mocks")]

use hashing::{
    AdmitHashingCompositionDepsOverrides, AdmitHashingCompositionParamsOverrides,
    AdmitHashingCompositionSuccessReturn, COMMITMENT_CONCRETES,
    LookupHashingDeclarationParamsOverrides, admit_hashing_composition,
    build_admit_hashing_composition_deps, build_admit_hashing_composition_params,
    build_admit_hashing_composition_payload, build_lookup_hashing_declaration_deps,
    build_lookup_hashing_declaration_params, build_lookup_hashing_declaration_payload,
    lookup_hashing_declaration,
};

/// Contract: `admit_hashing_composition` is called through the crate's public
///   surface once for each member of `COMMITMENT_CONCRETES`, the deps holding
///   the real `lookup_hashing_declaration` and the params naming the member and
///   the identifier the real lookup returns for that member →
///   `Ok(AdmitHashingCompositionSuccessReturn)`, so every declared member is
///   admitted for the identifier it declares.
/// Arrange: for each member of `COMMITMENT_CONCRETES`,
///   `build_admit_hashing_composition_deps` with the
///   `lookup_hashing_declaration` override `lookup_hashing_declaration`, the
///   real function; the declaration the real `lookup_hashing_declaration`
///   returns for that member, read through
///   `build_lookup_hashing_declaration_deps`,
///   `build_lookup_hashing_declaration_params` with the `concrete` override
///   that member, and `build_lookup_hashing_declaration_payload`, bound
///   through the irrefutable pattern `let Ok(success) = result;`; and
///   `build_admit_hashing_composition_params` with the `concrete` override
///   that member and the `identifier` override that declaration's
///   `identifier`, the identifier the member provides, read where the member
///   is received and never restated; the body is written once and run over
///   the declared set.
/// Act:     `admit_hashing_composition(&deps, params, payload)` once per
///   member.
/// Assert:  for each member the result equals
///   `Ok(AdmitHashingCompositionSuccessReturn)`.
/// Boundary: the public call `admit_hashing_composition` from the crate root,
///   running the real injected `lookup_hashing_declaration`, the `match` arm
///   of the selected member, and that member's interface-held declaration
///   constant.
/// Mocked:  nothing.
#[test]
fn every_declared_member_declares_itself_through_the_public_surface() {
    for member in COMMITMENT_CONCRETES {
        // Arrange
        let deps = build_admit_hashing_composition_deps(AdmitHashingCompositionDepsOverrides {
            lookup_hashing_declaration: Some(lookup_hashing_declaration),
        });
        let lookup_result = lookup_hashing_declaration(
            &build_lookup_hashing_declaration_deps(Default::default()),
            build_lookup_hashing_declaration_params(LookupHashingDeclarationParamsOverrides {
                concrete: Some(*member),
            }),
            build_lookup_hashing_declaration_payload(Default::default()),
        );
        let Ok(success) = lookup_result;
        let params =
            build_admit_hashing_composition_params(AdmitHashingCompositionParamsOverrides {
                concrete: Some(*member),
                identifier: Some(success.declaration.identifier),
            });
        let payload = build_admit_hashing_composition_payload(Default::default());

        // Act
        let result = admit_hashing_composition(&deps, params, payload);

        // Assert
        assert_eq!(result, Ok(AdmitHashingCompositionSuccessReturn));
    }
}
