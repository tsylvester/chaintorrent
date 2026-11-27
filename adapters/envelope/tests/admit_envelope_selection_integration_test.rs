//! Public integration test for `admit_envelope_selection`: proves, through
//! the `envelope` crate's re-exported surface, the own entries
//! `every_admitted_key_agreement_selection_declares_the_member_it_names` and
//! `every_declared_key_agreement_member_refuses_a_supplied_setting`, each by
//! the block of the same name.

#![cfg(feature = "mocks")]
#![allow(clippy::expect_used)]

use envelope::{
    AdmitEnvelopeSelectionErrorReturn, AdmitEnvelopeSelectionParamsOverrides,
    KEY_AGREEMENT_CONCRETES, KeyAgreementConcreteOverrides, admit_envelope_selection,
    build_admit_envelope_selection_deps, build_admit_envelope_selection_params,
    build_key_agreement_concrete,
};
use serde_json::{Map, Value};

/// Contract: `admit_envelope_selection` is called through the crate's public
///   surface with the member of each member of `KEY_AGREEMENT_CONCRETES` and
///   a document that supplies no setting → `Ok` whose `selection` equals the
///   member the params carried.
/// Arrange: for each `member` in `KEY_AGREEMENT_CONCRETES`,
///   `build_admit_envelope_selection_params` with the `concrete` override
///   `build_key_agreement_concrete(member, KeyAgreementConcreteOverrides)`;
///   `build_admit_envelope_selection_deps` with no override; the document
///   `Value::Object` over an empty `serde_json::Map`, written directly; the
///   body is written once and runs over the declared set, and the call is
///   made from outside the crate with no member constructed.
/// Act:     `admit_envelope_selection(&deps, params, document)` for the case.
/// Assert:  the success arm is bound through `expect` naming the invariant
///   that a document holding no key supplies no setting; `success.selection`
///   equals the member, the independent `params.concrete`.
/// Boundary: the public call `admit_envelope_selection`; the real chain is
///   that function.
/// Mocked:  nothing; the test constructs no member.
#[test]
fn every_admitted_key_agreement_selection_declares_the_member_it_names() {
    // Arrange
    for concrete in KEY_AGREEMENT_CONCRETES {
        let member = build_key_agreement_concrete(*concrete, KeyAgreementConcreteOverrides);
        let deps = build_admit_envelope_selection_deps(Default::default());
        let params = build_admit_envelope_selection_params(AdmitEnvelopeSelectionParamsOverrides {
            concrete: Some(member),
        });
        let document = Value::Object(Map::new());

        // Act
        let result = admit_envelope_selection(&deps, params, document);

        // Assert
        let success = result.expect("a document holding no key supplies no setting");
        assert_eq!(success.selection, member);
    }
}

/// Contract: `admit_envelope_selection` is called through the crate's public
///   surface with the member of each member of `KEY_AGREEMENT_CONCRETES` and
///   a document that supplies a setting →
///   `Err(AdmitEnvelopeSelectionErrorReturn::SettingSupplied { concrete })`
///   with `concrete` the admitted member, and no selection returned.
/// Arrange: for each `member` in `KEY_AGREEMENT_CONCRETES`,
///   `build_admit_envelope_selection_params` with the `concrete` override
///   `build_key_agreement_concrete(member, KeyAgreementConcreteOverrides)`;
///   `build_admit_envelope_selection_deps` with no override; the document
///   `Value::Object` over a `serde_json::Map` holding the key `setting` with
///   the integer value one, written directly.
/// Act:     `admit_envelope_selection(&deps, params, document)` for the case.
/// Assert:  `result.err()` equals
///   `Some(AdmitEnvelopeSelectionErrorReturn::SettingSupplied { concrete })`,
///   the whole error.
/// Boundary: the public call `admit_envelope_selection`; the real chain is
///   that function.
/// Mocked:  nothing; the test constructs no member.
#[test]
fn every_declared_key_agreement_member_refuses_a_supplied_setting() {
    // Arrange
    for concrete in KEY_AGREEMENT_CONCRETES {
        let member = build_key_agreement_concrete(*concrete, KeyAgreementConcreteOverrides);
        let deps = build_admit_envelope_selection_deps(Default::default());
        let params = build_admit_envelope_selection_params(AdmitEnvelopeSelectionParamsOverrides {
            concrete: Some(member),
        });
        let mut map = Map::new();
        map.insert("setting".to_string(), Value::from(1));
        let document = Value::Object(map);

        // Act
        let result = admit_envelope_selection(&deps, params, document);

        // Assert
        assert_eq!(
            result.err(),
            Some(AdmitEnvelopeSelectionErrorReturn::SettingSupplied { concrete: member })
        );
    }
}
