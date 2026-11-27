#![cfg(feature = "mocks")]

use envelope::{
    ENVELOPE_ALGEBRAS, KEY_AGREEMENT_CONCRETES, KEY_AGREEMENT_IDENTIFIERS,
    KeyAgreementConcreteOverrides, LookupEnvelopeDeclarationParamsOverrides,
    build_key_agreement_concrete, build_lookup_envelope_declaration_deps,
    build_lookup_envelope_declaration_params, build_lookup_envelope_declaration_payload,
    lookup_envelope_declaration,
};

/// Contract: own entry `every_declared_key_agreement_member_declares_itself`;
///   `lookup_envelope_declaration` called with each member of
///   `KEY_AGREEMENT_CONCRETES`, the selection `build_key_agreement_concrete`
///   returns for that member → `Ok` holding a declaration whose `concrete`
///   equals the member looked up, whose `identifier` is a member of
///   `KEY_AGREEMENT_IDENTIFIERS`, and whose `algebra` is a member of
///   `ENVELOPE_ALGEBRAS`.
/// Arrange:  the params from `build_lookup_envelope_declaration_params` with
///   `concrete` the selection `build_key_agreement_concrete` returns for each
///   member of `KEY_AGREEMENT_CONCRETES` under `KeyAgreementConcreteOverrides`,
///   the body written once and run over the declared set; the deps and payload
///   from their builders.
/// Act:      `lookup_envelope_declaration` once per member.
/// Assert:   each result binds through the irrefutable `let Ok(success) = …;`;
///   the declaration's `concrete` equals the member, `KEY_AGREEMENT_IDENTIFIERS`
///   contains its `identifier`, and `ENVELOPE_ALGEBRAS` contains its `algebra`.
/// Boundary: the public call `lookup_envelope_declaration`; the real chain is
///   that function alone.
/// Mocked:   none; no outer-edge collaborator is on the route.
#[test]
fn every_declared_key_agreement_member_declares_itself() {
    for member in KEY_AGREEMENT_CONCRETES {
        // Arrange
        let deps = build_lookup_envelope_declaration_deps(Default::default());
        let params =
            build_lookup_envelope_declaration_params(LookupEnvelopeDeclarationParamsOverrides {
                concrete: Some(build_key_agreement_concrete(
                    *member,
                    KeyAgreementConcreteOverrides,
                )),
            });
        let payload = build_lookup_envelope_declaration_payload(Default::default());

        // Act
        let result = lookup_envelope_declaration(&deps, params, payload);

        // Assert
        let Ok(success) = result;
        assert_eq!(success.declaration.concrete, *member);
        assert!(KEY_AGREEMENT_IDENTIFIERS.contains(&success.declaration.identifier));
        assert!(ENVELOPE_ALGEBRAS.contains(&success.declaration.algebra));
    }
}
// every_declared_key_agreement_member_declares_itself

// Public-surface enumeration: the own entry
//   `every_declared_key_agreement_member_declares_itself`,
//   proven by `every_declared_key_agreement_member_declares_itself`.
