#![allow(clippy::expect_used)]

use super::Keccak256HashToScalar;
use super::mock::build_keccak256_hash_to_scalar_constructor_params;
use crate::factory::provides::{
    HASH_TO_SCALAR_CONCRETES, HASH_TO_SCALAR_IDENTIFIERS, HASH_TO_SCALAR_INTERFACE_VERSION,
    HashToScalarConcrete, HashToScalarConcreteOverrides, HashToScalarDeclaration,
    HashToScalarIdentifier, build_hash_to_scalar_concrete,
};

/// Contract: `try_new` over fieldless params yields `Ok(Keccak256HashToScalar)`.
/// Arrange: params from `build_keccak256_hash_to_scalar_constructor_params(Default::default())`.
/// Act:     `Keccak256HashToScalar::try_new` over those params.
/// Assert:  `result.is_ok()`.
#[test]
fn try_new_returns_the_adapter() {
    // Arrange
    let params = build_keccak256_hash_to_scalar_constructor_params(Default::default());

    // Act
    let result = Keccak256HashToScalar::try_new(params);

    // Assert
    assert!(result.is_ok());
}

/// Contract: a direct read of `Keccak256HashToScalar::DECLARATION` →
///   `HashToScalarDeclaration { concrete: HashToScalarConcrete::Keccak256,
///   identifier: HashToScalarIdentifier::Keccak256V1, adapter_version: 1,
///   interface_version: HASH_TO_SCALAR_INTERFACE_VERSION }`, no adapter
///   constructed.
/// Arrange: none; `Keccak256HashToScalar::DECLARATION` is the
///   interface-held constant.
/// Act:     `Keccak256HashToScalar::DECLARATION` is read.
/// Assert:  the concrete matches `HashToScalarConcrete::Keccak256`, the
///   identifier matches `HashToScalarIdentifier::Keccak256V1`, the adapter
///   version equals the literal `1`, and the interface version equals
///   `HASH_TO_SCALAR_INTERFACE_VERSION`.
#[test]
fn declaration_returns_the_keccak256_declaration() {
    // Arrange

    // Act
    let declaration: HashToScalarDeclaration = Keccak256HashToScalar::DECLARATION;

    // Assert
    assert!(matches!(
        declaration.concrete,
        HashToScalarConcrete::Keccak256
    ));
    assert!(matches!(
        declaration.identifier,
        HashToScalarIdentifier::Keccak256V1
    ));
    assert_eq!(declaration.adapter_version, 1);
    assert_eq!(
        declaration.interface_version,
        HASH_TO_SCALAR_INTERFACE_VERSION
    );
}

/// Contract: `HASH_TO_SCALAR_CONCRETES` contains only
///   `HashToScalarConcrete::Keccak256`; `HASH_TO_SCALAR_IDENTIFIERS`
///   contains only `HashToScalarIdentifier::Keccak256V1`; every member and
///   value `name()` spelling is distinct; `Keccak256V1` spells
///   `keccak256_v1` with `uint8` wire code `1`; no `Mock` value variant or
///   mock member exists in a real set; `build_hash_to_scalar_concrete`
///   returns the real selection without configuration.
/// Arrange: `HASH_TO_SCALAR_CONCRETES` and `HASH_TO_SCALAR_IDENTIFIERS`;
///   `build_hash_to_scalar_concrete` over `HashToScalarConcrete::Keccak256`
///   with fieldless overrides.
/// Act:     each declared set and each member's and value's `name()` is
///   read; the declared-selection builder is run.
/// Assert:  the concrete set is exactly `Keccak256` and the identifier set
///   exactly `Keccak256V1`; the spellings are pairwise distinct;
///   `Keccak256V1` spells `keccak256_v1` and its `uint8` wire code is `1`;
///   no `Mock` exists in the value identifier and no mock member appears in
///   a real set; the builder returns `HashToScalarConcrete::Keccak256`
///   carrying no configuration.
#[test]
fn the_declared_sets_and_spellings_are_total_and_real() {
    // Arrange

    // Act
    let selection = build_hash_to_scalar_concrete(
        HashToScalarConcrete::Keccak256,
        HashToScalarConcreteOverrides,
    );

    // Assert
    assert!(matches!(
        HASH_TO_SCALAR_CONCRETES,
        [HashToScalarConcrete::Keccak256]
    ));
    assert!(matches!(
        HASH_TO_SCALAR_IDENTIFIERS,
        [HashToScalarIdentifier::Keccak256V1]
    ));
    let spellings = [
        HashToScalarConcrete::Keccak256.name(),
        HashToScalarIdentifier::Keccak256V1.name(),
    ];
    for (index, spelling) in spellings.iter().enumerate() {
        assert!(spellings[..index].iter().all(|earlier| earlier != spelling));
    }
    assert_eq!(HashToScalarIdentifier::Keccak256V1.name(), "keccak256_v1");
    assert_eq!(HashToScalarIdentifier::Keccak256V1.wire_code(), 1);
    assert!(matches!(selection, HashToScalarConcrete::Keccak256));
}
