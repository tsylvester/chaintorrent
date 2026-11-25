#![cfg(feature = "mocks")]

use encoding::{
    ENCODING_CONCRETES, ENCODING_IDENTIFIERS, ENCODING_INTERFACE_VERSION,
    EncodingConcreteOverrides, LookupEncodingDeclarationDepsOverrides,
    LookupEncodingDeclarationParamsOverrides, LookupEncodingDeclarationPayloadOverrides,
    build_encoding_concrete, build_lookup_encoding_declaration_deps,
    build_lookup_encoding_declaration_params, build_lookup_encoding_declaration_payload,
    lookup_encoding_declaration,
};

/// Contract: own entry `declared_member_declaration_through_the_public_surface`: for
///   each member of `ENCODING_CONCRETES`, `lookup_encoding_declaration` through the
///   crate's public surface → `Ok` of that member's own declaration.
/// Arrange: for each member of `ENCODING_CONCRETES`, the params from
///   `build_lookup_encoding_declaration_params` overriding `concrete` with
///   `build_encoding_concrete(member, EncodingConcreteOverrides::default())`; the deps
///   and payload from their builders; the declared members are the input variation.
/// Act: `lookup_encoding_declaration(&deps, params, payload)` for each member, imported
///   from the crate root.
/// Assert: for each member, the declaration's `concrete` equals the member passed, its
///   `identifier` is contained in `ENCODING_IDENTIFIERS`, and its `interface_version`
///   equals `ENCODING_INTERFACE_VERSION`, each by `assert_eq!` or `assert!`, each
///   compared apart.
/// Boundary: the crate root's public call `lookup_encoding_declaration` and the route
///   from the crate barrel through the lookup to the member's declaration constant.
/// Mocked: none; the official builders construct the inputs, and no outer-edge
///   collaborator lies on the route.
#[test]
fn lookup_returns_each_declared_member_its_own_declaration() {
    for member in ENCODING_CONCRETES {
        // Arrange
        let deps = build_lookup_encoding_declaration_deps(LookupEncodingDeclarationDepsOverrides);
        let params =
            build_lookup_encoding_declaration_params(LookupEncodingDeclarationParamsOverrides {
                concrete: Some(build_encoding_concrete(
                    *member,
                    EncodingConcreteOverrides::default(),
                )),
            });
        let payload =
            build_lookup_encoding_declaration_payload(LookupEncodingDeclarationPayloadOverrides);

        // Act
        let result = lookup_encoding_declaration(&deps, params, payload);

        // Assert
        let success = result.ok();
        assert_eq!(
            success.as_ref().map(|success| success.declaration.concrete),
            Some(*member)
        );
        assert!(success.as_ref().is_some_and(|success| {
            ENCODING_IDENTIFIERS.contains(&success.declaration.identifier)
        }));
        assert_eq!(
            success
                .as_ref()
                .map(|success| success.declaration.interface_version),
            Some(ENCODING_INTERFACE_VERSION)
        );
    }
}
