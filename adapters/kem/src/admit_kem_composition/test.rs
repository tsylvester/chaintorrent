#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::admit_kem_composition;
use super::interface::{AdmitKemCompositionErrorReturn, AdmitKemCompositionSuccessReturn};
use super::mock::{
    AdmitKemCompositionDepsOverrides, AdmitKemCompositionParamsOverrides,
    build_admit_kem_composition_deps, build_admit_kem_composition_params,
    build_admit_kem_composition_payload,
};
use crate::factory::provides::{
    IdentityScope, KemConcrete, KemDeclarationOverrides, build_kem_declaration,
};
use crate::lookup_kem_declaration::provides::{
    LookupKemDeclarationDeps, LookupKemDeclarationParams, LookupKemDeclarationPayload,
    LookupKemDeclarationReturn, LookupKemDeclarationSuccessReturn,
};

/// Contract: `admit_kem_composition`, scope provided: the declaration's
///   `identity_scopes` contains `params.scope` → `Ok` naming the member and
///   the scope.
/// Arrange: the deps from `build_admit_kem_composition_deps` overriding
///   `lookup_kem_declaration` with a function returning `Ok` around
///   `build_kem_declaration` with `identity_scopes` `&[IdentityScope::Asset]`,
///   the params from `build_admit_kem_composition_params` with `concrete`
///   `KemConcrete::Bb1DepthOne` and `scope` `IdentityScope::Asset`, and the
///   payload from its builder, the declaration providing a scope the refusal
///   blocks below leave out.
/// Act:     `admit_kem_composition` once.
/// Assert:  the whole result is
///   `Ok(AdmitKemCompositionSuccessReturn { concrete: KemConcrete::Bb1DepthOne, scope: IdentityScope::Asset })`.
#[test]
fn admits_a_scope_the_declaration_provides() {
    // Arrange
    fn lookup_providing_asset(
        _deps: &LookupKemDeclarationDeps,
        _params: LookupKemDeclarationParams,
        _payload: LookupKemDeclarationPayload,
    ) -> LookupKemDeclarationReturn {
        Ok(LookupKemDeclarationSuccessReturn {
            declaration: build_kem_declaration(KemDeclarationOverrides {
                identity_scopes: Some(&[IdentityScope::Asset]),
                ..Default::default()
            }),
        })
    }
    let deps = build_admit_kem_composition_deps(AdmitKemCompositionDepsOverrides {
        lookup_kem_declaration: Some(lookup_providing_asset),
    });
    let params = build_admit_kem_composition_params(AdmitKemCompositionParamsOverrides {
        concrete: Some(KemConcrete::Bb1DepthOne),
        scope: Some(IdentityScope::Asset),
    });
    let payload = build_admit_kem_composition_payload(Default::default());

    // Act
    let result = admit_kem_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(AdmitKemCompositionSuccessReturn {
            concrete: KemConcrete::Bb1DepthOne,
            scope: IdentityScope::Asset,
        })
    );
}
// admits_a_scope_the_declaration_provides

/// Contract: `admit_kem_composition`, scope provided for a declaration naming
///   more than one scope, the required scope following another entry → `Ok`
///   naming the scope.
/// Arrange: as `admits_a_scope_the_declaration_provides` with `identity_scopes`
///   `&[IdentityScope::Entitlement, IdentityScope::Asset]`, a scope that
///   follows `IdentityScope::Entitlement` in the slice, against the block
///   above whose scope is its only entry, so a branch that compared the scope
///   with the front entry of the slice alone fails.
/// Act:     `admit_kem_composition` once.
/// Assert:  the whole result is
///   `Ok(AdmitKemCompositionSuccessReturn { concrete: KemConcrete::Bb1DepthOne, scope: IdentityScope::Asset })`.
#[test]
fn admits_a_scope_that_is_a_later_entry_of_a_declaration_that_provides_both() {
    // Arrange
    fn lookup_providing_both(
        _deps: &LookupKemDeclarationDeps,
        _params: LookupKemDeclarationParams,
        _payload: LookupKemDeclarationPayload,
    ) -> LookupKemDeclarationReturn {
        Ok(LookupKemDeclarationSuccessReturn {
            declaration: build_kem_declaration(KemDeclarationOverrides {
                identity_scopes: Some(&[IdentityScope::Entitlement, IdentityScope::Asset]),
                ..Default::default()
            }),
        })
    }
    let deps = build_admit_kem_composition_deps(AdmitKemCompositionDepsOverrides {
        lookup_kem_declaration: Some(lookup_providing_both),
    });
    let params = build_admit_kem_composition_params(AdmitKemCompositionParamsOverrides {
        concrete: Some(KemConcrete::Bb1DepthOne),
        scope: Some(IdentityScope::Asset),
    });
    let payload = build_admit_kem_composition_payload(Default::default());

    // Act
    let result = admit_kem_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(AdmitKemCompositionSuccessReturn {
            concrete: KemConcrete::Bb1DepthOne,
            scope: IdentityScope::Asset,
        })
    );
}
// admits_a_scope_that_is_a_later_entry_of_a_declaration_that_provides_both

/// Contract: `admit_kem_composition`, scope not provided: the declaration's
///   `identity_scopes` does not contain `params.scope` → `Err(Incompatible)`
///   naming the member, the declared scopes, and the required scope.
/// Arrange: as `admits_a_scope_the_declaration_provides` with `identity_scopes`
///   `&[IdentityScope::Asset]` and `scope` `IdentityScope::Entitlement`, the
///   variation separating it from the admitted blocks.
/// Act:     `admit_kem_composition` once.
/// Assert:  the whole result is
///   `Err(AdmitKemCompositionErrorReturn::Incompatible { concrete: KemConcrete::Bb1DepthOne, declared: &[IdentityScope::Asset], required: IdentityScope::Entitlement })`.
#[test]
fn refuses_the_entitlement_scope_when_the_declaration_provides_only_the_asset_scope() {
    // Arrange
    fn lookup_providing_asset(
        _deps: &LookupKemDeclarationDeps,
        _params: LookupKemDeclarationParams,
        _payload: LookupKemDeclarationPayload,
    ) -> LookupKemDeclarationReturn {
        Ok(LookupKemDeclarationSuccessReturn {
            declaration: build_kem_declaration(KemDeclarationOverrides {
                identity_scopes: Some(&[IdentityScope::Asset]),
                ..Default::default()
            }),
        })
    }
    let deps = build_admit_kem_composition_deps(AdmitKemCompositionDepsOverrides {
        lookup_kem_declaration: Some(lookup_providing_asset),
    });
    let params = build_admit_kem_composition_params(AdmitKemCompositionParamsOverrides {
        concrete: Some(KemConcrete::Bb1DepthOne),
        scope: Some(IdentityScope::Entitlement),
    });
    let payload = build_admit_kem_composition_payload(Default::default());

    // Act
    let result = admit_kem_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitKemCompositionErrorReturn::Incompatible {
            concrete: KemConcrete::Bb1DepthOne,
            declared: &[IdentityScope::Asset],
            required: IdentityScope::Entitlement,
        })
    );
}
// refuses_the_entitlement_scope_when_the_declaration_provides_only_the_asset_scope

/// Contract: `admit_kem_composition`, scope not provided for the other scope:
///   the declaration's `identity_scopes` provides only
///   `IdentityScope::Entitlement` → `Err(Incompatible)`.
/// Arrange: as `refuses_the_entitlement_scope_when_the_declaration_provides_only_the_asset_scope`
///   with `identity_scopes` `&[IdentityScope::Entitlement]` and `scope`
///   `IdentityScope::Asset`, so a branch that refused one fixed scope fails.
/// Act:     `admit_kem_composition` once.
/// Assert:  the whole result is
///   `Err(AdmitKemCompositionErrorReturn::Incompatible { concrete: KemConcrete::Bb1DepthOne, declared: &[IdentityScope::Entitlement], required: IdentityScope::Asset })`.
#[test]
fn refuses_the_asset_scope_when_the_declaration_provides_only_the_entitlement_scope() {
    // Arrange
    fn lookup_providing_entitlement(
        _deps: &LookupKemDeclarationDeps,
        _params: LookupKemDeclarationParams,
        _payload: LookupKemDeclarationPayload,
    ) -> LookupKemDeclarationReturn {
        Ok(LookupKemDeclarationSuccessReturn {
            declaration: build_kem_declaration(KemDeclarationOverrides {
                identity_scopes: Some(&[IdentityScope::Entitlement]),
                ..Default::default()
            }),
        })
    }
    let deps = build_admit_kem_composition_deps(AdmitKemCompositionDepsOverrides {
        lookup_kem_declaration: Some(lookup_providing_entitlement),
    });
    let params = build_admit_kem_composition_params(AdmitKemCompositionParamsOverrides {
        concrete: Some(KemConcrete::Bb1DepthOne),
        scope: Some(IdentityScope::Asset),
    });
    let payload = build_admit_kem_composition_payload(Default::default());

    // Act
    let result = admit_kem_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitKemCompositionErrorReturn::Incompatible {
            concrete: KemConcrete::Bb1DepthOne,
            declared: &[IdentityScope::Entitlement],
            required: IdentityScope::Asset,
        })
    );
}
// refuses_the_asset_scope_when_the_declaration_provides_only_the_entitlement_scope

/// Contract: `admit_kem_composition`, the declaration read is the one the
///   params' member names → `Ok` for a member whose declaration provides the
///   scope.
/// Arrange: the deps with a function that compares `params.concrete` with
///   `KemConcrete::Mock` and returns `&[IdentityScope::Entitlement,
///   IdentityScope::Asset]` when they are equal and `&[IdentityScope::Asset]`
///   otherwise, holding no arm per member of the family, and the params with
///   `concrete` `KemConcrete::Mock` and `scope` `IdentityScope::Entitlement`.
/// Act:     `admit_kem_composition` once.
/// Assert:  the whole result is
///   `Ok(AdmitKemCompositionSuccessReturn { concrete: KemConcrete::Mock, scope: IdentityScope::Entitlement })`,
///   so a branch that read another member's declaration fails.
#[test]
fn reads_the_declaration_of_the_member_the_params_carry_and_admits_it() {
    // Arrange
    fn lookup_by_member(
        _deps: &LookupKemDeclarationDeps,
        params: LookupKemDeclarationParams,
        _payload: LookupKemDeclarationPayload,
    ) -> LookupKemDeclarationReturn {
        let identity_scopes = if params.concrete == KemConcrete::Mock {
            &[IdentityScope::Entitlement, IdentityScope::Asset][..]
        } else {
            &[IdentityScope::Asset][..]
        };
        Ok(LookupKemDeclarationSuccessReturn {
            declaration: build_kem_declaration(KemDeclarationOverrides {
                identity_scopes: Some(identity_scopes),
                ..Default::default()
            }),
        })
    }
    let deps = build_admit_kem_composition_deps(AdmitKemCompositionDepsOverrides {
        lookup_kem_declaration: Some(lookup_by_member),
    });
    let params = build_admit_kem_composition_params(AdmitKemCompositionParamsOverrides {
        concrete: Some(KemConcrete::Mock),
        scope: Some(IdentityScope::Entitlement),
    });
    let payload = build_admit_kem_composition_payload(Default::default());

    // Act
    let result = admit_kem_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Ok(AdmitKemCompositionSuccessReturn {
            concrete: KemConcrete::Mock,
            scope: IdentityScope::Entitlement,
        })
    );
}
// reads_the_declaration_of_the_member_the_params_carry_and_admits_it

/// Contract: `admit_kem_composition`, the same read for the member whose
///   declaration lacks the scope → `Err(Incompatible)`.
/// Arrange: as `reads_the_declaration_of_the_member_the_params_carry_and_admits_it`
///   with the same lookup function and the params with `concrete`
///   `KemConcrete::Bb1DepthOne` and `scope` `IdentityScope::Entitlement`.
/// Act:     `admit_kem_composition` once.
/// Assert:  the whole result is
///   `Err(AdmitKemCompositionErrorReturn::Incompatible { concrete: KemConcrete::Bb1DepthOne, declared: &[IdentityScope::Asset], required: IdentityScope::Entitlement })`.
#[test]
fn reads_the_declaration_of_the_member_the_params_carry_and_refuses_it() {
    // Arrange
    fn lookup_by_member(
        _deps: &LookupKemDeclarationDeps,
        params: LookupKemDeclarationParams,
        _payload: LookupKemDeclarationPayload,
    ) -> LookupKemDeclarationReturn {
        let identity_scopes = if params.concrete == KemConcrete::Mock {
            &[IdentityScope::Entitlement, IdentityScope::Asset][..]
        } else {
            &[IdentityScope::Asset][..]
        };
        Ok(LookupKemDeclarationSuccessReturn {
            declaration: build_kem_declaration(KemDeclarationOverrides {
                identity_scopes: Some(identity_scopes),
                ..Default::default()
            }),
        })
    }
    let deps = build_admit_kem_composition_deps(AdmitKemCompositionDepsOverrides {
        lookup_kem_declaration: Some(lookup_by_member),
    });
    let params = build_admit_kem_composition_params(AdmitKemCompositionParamsOverrides {
        concrete: Some(KemConcrete::Bb1DepthOne),
        scope: Some(IdentityScope::Entitlement),
    });
    let payload = build_admit_kem_composition_payload(Default::default());

    // Act
    let result = admit_kem_composition(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitKemCompositionErrorReturn::Incompatible {
            concrete: KemConcrete::Bb1DepthOne,
            declared: &[IdentityScope::Asset],
            required: IdentityScope::Entitlement,
        })
    );
}
// reads_the_declaration_of_the_member_the_params_carry_and_refuses_it
