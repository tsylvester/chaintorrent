#![allow(clippy::expect_used)]

use pairing::PairingCurve;
use std::collections::BTreeMap;

use super::admit_generate;
use super::interface::{AdmitGenerateErrorReturn, CompilerVersionErrorReturn};
use super::mock::{
    build_admit_generate_deps, build_admit_generate_params, build_admit_generate_payload,
    declared_name,
};
use crate::generate::provides::{
    GenerateConcrete, GeneratedDirectoryTryNewErrorReturn, GeneratedFileNameTryNewErrorReturn,
    MockIGenerateAdapterFailureMode, MsmTermCountTryNewErrorReturn,
    SolidityConstantNameTryNewErrorReturn, SolidityLibraryNameTryNewErrorReturn,
    SolidityMemberNameTryNewErrorReturn, SolidityTypeNameTryNewErrorReturn,
    SpdxLicenseIdentifierTryNewErrorReturn, VectorCountTryNewErrorReturn, VectorKind,
};

/// Contract: Concrete → `Err(AdmitGenerateErrorReturn::Concrete { text })`.
/// Arrange: the default payload with `concrete` `"unknown"`.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::Concrete { text:
///   "unknown".to_string() })`.
#[test]
fn admit_generate_refuses_an_unknown_concrete() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    payload.generate.concrete = "unknown".to_string();
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::Concrete {
            text: "unknown".to_string()
        })
    );
    // admit_generate_refuses_an_unknown_concrete
}

/// Contract: Concrete → the family's mock selection is never a member of the
///   declared set.
/// Arrange: the default payload with `concrete` `declared_name` of
///   `GenerateConcrete::Mock(MockIGenerateAdapterFailureMode::NoFailure)`.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::Concrete { text })`
///   with `text` that name.
#[test]
fn admit_generate_refuses_the_mock_concretes_name() {
    // Arrange
    let text = declared_name(&GenerateConcrete::Mock(
        MockIGenerateAdapterFailureMode::NoFailure,
    ));
    let mut payload = build_admit_generate_payload(Default::default());
    payload.generate.concrete = text.clone();
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(result, Err(AdmitGenerateErrorReturn::Concrete { text }));
    // admit_generate_refuses_the_mock_concretes_name
}

/// Contract: Kinds → `Err(AdmitGenerateErrorReturn::Kind { text })`.
/// Arrange: the default payload with the entry's key `"unknown"`.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::Kind { text:
///   "unknown".to_string() })`.
#[test]
fn admit_generate_refuses_an_unknown_kind() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    let entry = payload
        .generate
        .kinds
        .values_mut()
        .next()
        .expect("one entry");
    let entry = entry.clone();
    payload.generate.kinds = BTreeMap::from([("unknown".to_string(), entry)]);
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::Kind {
            text: "unknown".to_string()
        })
    );
    // admit_generate_refuses_an_unknown_kind
}

/// Contract: the naming rule → a text equals a member's `Debug` text whole
///   and case-sensitively.
/// Arrange: the default payload with the entry's key the lowercase of
///   `declared_name(&VectorKind::G1Addition)`.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::Kind { text })`
///   with `text` the lowercase text.
#[test]
fn admit_generate_refuses_a_kind_name_that_differs_only_in_case() {
    // Arrange
    let text = declared_name(&VectorKind::G1Addition).to_lowercase();
    let mut payload = build_admit_generate_payload(Default::default());
    let entry = payload
        .generate
        .kinds
        .values_mut()
        .next()
        .expect("one entry");
    let entry = entry.clone();
    payload.generate.kinds = BTreeMap::from([(text.clone(), entry)]);
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(result, Err(AdmitGenerateErrorReturn::Kind { text }));
    // admit_generate_refuses_a_kind_name_that_differs_only_in_case
}

/// Contract: Count → `Err(AdmitGenerateErrorReturn::Count { kind, error })`,
///   the constructor's refusal unchanged.
/// Arrange: the default payload with the entry's `count` `Some(0)`.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::Count { kind:
///   declared_name(&VectorKind::G1Addition), error:
///   VectorCountTryNewErrorReturn::Zero })`.
#[test]
fn admit_generate_refuses_a_zero_count_with_its_kind() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    payload
        .generate
        .kinds
        .values_mut()
        .next()
        .expect("one entry")
        .count = Some(0);
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::Count {
            kind: declared_name(&VectorKind::G1Addition),
            error: VectorCountTryNewErrorReturn::Zero
        })
    );
    // admit_generate_refuses_a_zero_count_with_its_kind
}

/// Contract: Terms → `Err(AdmitGenerateErrorReturn::Terms { kind, error })`.
/// Arrange: the default payload with the entry's `terms` `Some(0)`.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::Terms { kind:
///   declared_name(&VectorKind::G1Addition), error:
///   MsmTermCountTryNewErrorReturn::Zero })`.
#[test]
fn admit_generate_refuses_a_zero_term_count_with_its_kind() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    payload
        .generate
        .kinds
        .values_mut()
        .next()
        .expect("one entry")
        .terms = Some(0);
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::Terms {
            kind: declared_name(&VectorKind::G1Addition),
            error: MsmTermCountTryNewErrorReturn::Zero
        })
    );
    // admit_generate_refuses_a_zero_term_count_with_its_kind
}

/// Contract: Names → `Err(AdmitGenerateErrorReturn::File { kind, error })`,
///   the constructor's refusal unchanged.
/// Arrange: the default payload with the entry's `file` `""`.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::File { kind:
///   declared_name(&VectorKind::G1Addition), error:
///   GeneratedFileNameTryNewErrorReturn::Empty })`.
#[test]
fn admit_generate_refuses_an_empty_file_name_with_its_kind() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    payload
        .generate
        .kinds
        .values_mut()
        .next()
        .expect("one entry")
        .file = String::new();
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::File {
            kind: declared_name(&VectorKind::G1Addition),
            error: GeneratedFileNameTryNewErrorReturn::Empty
        })
    );
    // admit_generate_refuses_an_empty_file_name_with_its_kind
}

/// Contract: Names → the refusal carries the index and byte the constructor
///   reports.
/// Arrange: the default payload with the entry's `file` `"a b"`.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::File { kind:
///   declared_name(&VectorKind::G1Addition), error:
///   GeneratedFileNameTryNewErrorReturn::InvalidCharacter { index: 1, byte: 32
///   } })`.
#[test]
fn admit_generate_refuses_a_file_name_with_an_invalid_byte() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    payload
        .generate
        .kinds
        .values_mut()
        .next()
        .expect("one entry")
        .file = "a b".to_string();
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::File {
            kind: declared_name(&VectorKind::G1Addition),
            error: GeneratedFileNameTryNewErrorReturn::InvalidCharacter { index: 1, byte: 32 }
        })
    );
    // admit_generate_refuses_a_file_name_with_an_invalid_byte
}

/// Contract: Names → `Err(AdmitGenerateErrorReturn::StructName { kind,
///   error })`.
/// Arrange: the default payload with the entry's `struct_name` `"1Vector"`.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::StructName {
///   kind: declared_name(&VectorKind::G1Addition), error:
///   SolidityTypeNameTryNewErrorReturn::LeadingCharacter { byte: 49 } })`.
#[test]
fn admit_generate_refuses_a_struct_name_with_an_invalid_leading_byte() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    payload
        .generate
        .kinds
        .values_mut()
        .next()
        .expect("one entry")
        .struct_name = "1Vector".to_string();
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::StructName {
            kind: declared_name(&VectorKind::G1Addition),
            error: SolidityTypeNameTryNewErrorReturn::LeadingCharacter { byte: 49 }
        })
    );
    // admit_generate_refuses_a_struct_name_with_an_invalid_leading_byte
}

/// Contract: Names → `Err` carrying `DecodeFunction` with the entry's key and
///   the constructor's `Empty` refusal.
/// Arrange: the default payload with the entry's `decode_function` `""`.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::DecodeFunction {
///   kind: declared_name(&VectorKind::G1Addition), error:
///   SolidityMemberNameTryNewErrorReturn::Empty })`.
#[test]
fn admit_generate_refuses_an_empty_decode_function_name() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    payload
        .generate
        .kinds
        .values_mut()
        .next()
        .expect("one entry")
        .decode_function = String::new();
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::DecodeFunction {
            kind: declared_name(&VectorKind::G1Addition),
            error: SolidityMemberNameTryNewErrorReturn::Empty
        })
    );
    // admit_generate_refuses_an_empty_decode_function_name
}

/// Contract: Names → `Err` carrying `PathConstant` with the entry's key and
///   the constructor's `Empty` refusal.
/// Arrange: the default payload with the entry's `path_constant` `""`.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::PathConstant {
///   kind: declared_name(&VectorKind::G1Addition), error:
///   SolidityConstantNameTryNewErrorReturn::Empty })`.
#[test]
fn admit_generate_refuses_an_empty_path_constant_name() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    payload
        .generate
        .kinds
        .values_mut()
        .next()
        .expect("one entry")
        .path_constant = String::new();
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::PathConstant {
            kind: declared_name(&VectorKind::G1Addition),
            error: SolidityConstantNameTryNewErrorReturn::Empty
        })
    );
    // admit_generate_refuses_an_empty_path_constant_name
}

/// Contract: Curve libraries → `Err` carrying `ConstantsLibraryFile` with the
///   curve and the constructor's `Empty` refusal.
/// Arrange: the default payload with `constants_library.file` `""` in the
///   `PairingCurve::Bls12381` entry's table.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::
///   ConstantsLibraryFile { curve: PairingCurve::Bls12381, error:
///   GeneratedFileNameTryNewErrorReturn::Empty })`.
#[test]
fn admit_generate_refuses_an_empty_constants_library_file() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    payload
        .generate
        .curve_libraries
        .get_mut(&declared_name(&PairingCurve::Bls12381))
        .expect("the curve's entry")
        .constants_library
        .file = String::new();
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::ConstantsLibraryFile {
            curve: PairingCurve::Bls12381,
            error: GeneratedFileNameTryNewErrorReturn::Empty
        })
    );
    // admit_generate_refuses_an_empty_constants_library_file
}

/// Contract: Curve libraries → `Err` carrying `ConstantsLibraryName` with the
///   curve and the constructor's `Empty` refusal.
/// Arrange: the default payload with `constants_library.library` `""` in the
///   `PairingCurve::Bls12381` entry's table.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::
///   ConstantsLibraryName { curve: PairingCurve::Bls12381, error:
///   SolidityLibraryNameTryNewErrorReturn::Empty })`.
#[test]
fn admit_generate_refuses_an_empty_constants_library_name() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    payload
        .generate
        .curve_libraries
        .get_mut(&declared_name(&PairingCurve::Bls12381))
        .expect("the curve's entry")
        .constants_library
        .library = String::new();
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::ConstantsLibraryName {
            curve: PairingCurve::Bls12381,
            error: SolidityLibraryNameTryNewErrorReturn::Empty
        })
    );
    // admit_generate_refuses_an_empty_constants_library_name
}

/// Contract: Curve libraries → `Err` carrying `RecordsLibraryFile` with the
///   curve and the constructor's `Empty` refusal.
/// Arrange: the default payload with `records_library.file` `""` in the
///   `PairingCurve::Bls12381` entry's table.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::
///   RecordsLibraryFile { curve: PairingCurve::Bls12381, error:
///   GeneratedFileNameTryNewErrorReturn::Empty })`.
#[test]
fn admit_generate_refuses_an_empty_records_library_file() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    payload
        .generate
        .curve_libraries
        .get_mut(&declared_name(&PairingCurve::Bls12381))
        .expect("the curve's entry")
        .records_library
        .file = String::new();
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::RecordsLibraryFile {
            curve: PairingCurve::Bls12381,
            error: GeneratedFileNameTryNewErrorReturn::Empty
        })
    );
    // admit_generate_refuses_an_empty_records_library_file
}

/// Contract: Curve libraries → `Err` carrying `RecordsLibraryName` with the
///   curve and the constructor's `Empty` refusal.
/// Arrange: the default payload with `records_library.library` `""` in the
///   `PairingCurve::Bls12381` entry's table.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::
///   RecordsLibraryName { curve: PairingCurve::Bls12381, error:
///   SolidityLibraryNameTryNewErrorReturn::Empty })`.
#[test]
fn admit_generate_refuses_an_empty_records_library_name() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    payload
        .generate
        .curve_libraries
        .get_mut(&declared_name(&PairingCurve::Bls12381))
        .expect("the curve's entry")
        .records_library
        .library = String::new();
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::RecordsLibraryName {
            curve: PairingCurve::Bls12381,
            error: SolidityLibraryNameTryNewErrorReturn::Empty
        })
    );
    // admit_generate_refuses_an_empty_records_library_name
}

/// Contract: Generated directory → `Err(AdmitGenerateErrorReturn::
///   GeneratedDirectorySegment { index, error })`.
/// Arrange: `generated_directory` of `"generated"` then `"a b"`.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::
///   GeneratedDirectorySegment { index: 1, error:
///   GeneratedFileNameTryNewErrorReturn::InvalidCharacter { index: 1, byte: 32
///   } })`.
#[test]
fn admit_generate_refuses_a_generated_directory_segment_with_its_index() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    payload.generate.generated_directory = vec!["generated".to_string(), "a b".to_string()];
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::GeneratedDirectorySegment {
            index: 1,
            error: GeneratedFileNameTryNewErrorReturn::InvalidCharacter { index: 1, byte: 32 }
        })
    );
    // admit_generate_refuses_a_generated_directory_segment_with_its_index
}

/// Contract: Generated directory → `Err(AdmitGenerateErrorReturn::
///   GeneratedDirectory { error })`.
/// Arrange: `generated_directory` empty.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::
///   GeneratedDirectory { error:
///   GeneratedDirectoryTryNewErrorReturn::Empty })`.
#[test]
fn admit_generate_refuses_a_generated_directory_with_no_segments() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    payload.generate.generated_directory = vec![];
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::GeneratedDirectory {
            error: GeneratedDirectoryTryNewErrorReturn::Empty
        })
    );
    // admit_generate_refuses_a_generated_directory_with_no_segments
}

/// Contract: Curve directories → `Err(AdmitGenerateErrorReturn::CurveKey
///   { text })`.
/// Arrange: the default payload with the first curve entry keyed `"unknown"`.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::CurveKey { text:
///   "unknown".to_string() })`.
#[test]
fn admit_generate_refuses_an_unknown_curve_key() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    payload.generate.curve_directories = BTreeMap::from([
        ("unknown".to_string(), "directory".to_string()),
        (
            declared_name(&PairingCurve::Bls12381),
            "bls12_381".to_string(),
        ),
    ]);
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::CurveKey {
            text: "unknown".to_string()
        })
    );
    // admit_generate_refuses_an_unknown_curve_key
}

/// Contract: the naming rule → a curve key equals a member's `Debug` text
///   whole and case-sensitively.
/// Arrange: the default payload with the `PairingCurve::Bn254` entry keyed by
///   the lowercase of its `declared_name`.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::CurveKey { text })`
///   with `text` the lowercase text.
#[test]
fn admit_generate_refuses_a_curve_key_that_differs_only_in_case() {
    // Arrange
    let text = declared_name(&PairingCurve::Bn254).to_lowercase();
    let mut payload = build_admit_generate_payload(Default::default());
    payload.generate.curve_directories = BTreeMap::from([
        (
            declared_name(&PairingCurve::Bls12381),
            "bls12_381".to_string(),
        ),
        (text.clone(), "bn254".to_string()),
    ]);
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(result, Err(AdmitGenerateErrorReturn::CurveKey { text }));
    // admit_generate_refuses_a_curve_key_that_differs_only_in_case
}

/// Contract: Curve directories → the pairing family's mock variant is never a
///   member of the declared set.
/// Arrange: the default payload with the first curve entry keyed
///   `declared_name(&PairingCurve::Mock)`.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::CurveKey { text })`
///   with `text` that name.
#[test]
fn admit_generate_refuses_the_mock_curves_name() {
    // Arrange
    let text = declared_name(&PairingCurve::Mock);
    let mut payload = build_admit_generate_payload(Default::default());
    payload.generate.curve_directories = BTreeMap::from([
        (
            declared_name(&PairingCurve::Bls12381),
            "bls12_381".to_string(),
        ),
        (text.clone(), "directory".to_string()),
    ]);
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(result, Err(AdmitGenerateErrorReturn::CurveKey { text }));
    // admit_generate_refuses_the_mock_curves_name
}

/// Contract: Curve directories → `Err(AdmitGenerateErrorReturn::
///   CurveDirectory { curve, error })`.
/// Arrange: the `PairingCurve::Bls12381` entry's value `""`.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::CurveDirectory {
///   curve: PairingCurve::Bls12381, error:
///   GeneratedFileNameTryNewErrorReturn::Empty })`.
#[test]
fn admit_generate_refuses_an_invalid_curve_directory_with_its_curve() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    *payload
        .generate
        .curve_directories
        .get_mut(&declared_name(&PairingCurve::Bls12381))
        .expect("the curve's entry") = String::new();
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::CurveDirectory {
            curve: PairingCurve::Bls12381,
            error: GeneratedFileNameTryNewErrorReturn::Empty
        })
    );
    // admit_generate_refuses_an_invalid_curve_directory_with_its_curve
}

/// Contract: Missing curve → `Err(AdmitGenerateErrorReturn::
///   MissingCurveDirectory { curve })`, the first declared curve with no
///   entry.
/// Arrange: `curve_directories` holding the `PairingCurve::Bls12381` entry
///   alone.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::
///   MissingCurveDirectory { curve: PairingCurve::Bn254 })`.
#[test]
fn admit_generate_refuses_a_declared_curve_with_no_directory() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    payload.generate.curve_directories = BTreeMap::from([(
        declared_name(&PairingCurve::Bls12381),
        "bls12_381".to_string(),
    )]);
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::MissingCurveDirectory {
            curve: PairingCurve::Bn254
        })
    );
    // admit_generate_refuses_a_declared_curve_with_no_directory
}

/// Contract: Compiler version → `Err(AdmitGenerateErrorReturn::
///   CompilerVersion { text, error })` with `CompilerVersionErrorReturn::
///   Parts { count }`.
/// Arrange: `compiler_version` `"0.8"`, then `"0.8.24.1"`.
/// Act:     `admit_generate`, once per text.
/// Assert:  the results equal `Err(AdmitGenerateErrorReturn::CompilerVersion
///   { text, error: CompilerVersionErrorReturn::Parts { count: 2 } })` and the
///   same with `count` `4`.
#[test]
fn admit_generate_refuses_a_compiler_version_without_three_parts() {
    // Arrange / Act / Assert — each text, its part count.
    for (text, count) in [("0.8", 2_usize), ("0.8.24.1", 4_usize)] {
        let mut payload = build_admit_generate_payload(Default::default());
        payload.generate.compiler_version = text.to_string();
        let result = admit_generate(
            &build_admit_generate_deps(Default::default()),
            build_admit_generate_params(Default::default()),
            payload,
        );
        assert_eq!(
            result,
            Err(AdmitGenerateErrorReturn::CompilerVersion {
                text: text.to_string(),
                error: CompilerVersionErrorReturn::Parts { count }
            })
        );
    }
    // admit_generate_refuses_a_compiler_version_without_three_parts
}

/// Contract: Compiler version → `CompilerVersionErrorReturn::Part { index,
///   text }`, the lowest refused part.
/// Arrange: `compiler_version` `"0.x.y"`, then `"0.8.70000"`.
/// Act:     `admit_generate`, once per text.
/// Assert:  the results equal `Err(AdmitGenerateErrorReturn::CompilerVersion
///   { .. })` with `Part { index: 1, text: "x".to_string() }` and with
///   `Part { index: 2, text: "70000".to_string() }`.
#[test]
fn admit_generate_refuses_a_compiler_version_part_that_is_not_a_u16() {
    // Arrange / Act / Assert — each version text, the index and text of its
    // lowest refused part.
    for (version, index, part) in [("0.x.y", 1_usize, "x"), ("0.8.70000", 2_usize, "70000")] {
        let mut payload = build_admit_generate_payload(Default::default());
        payload.generate.compiler_version = version.to_string();
        let result = admit_generate(
            &build_admit_generate_deps(Default::default()),
            build_admit_generate_params(Default::default()),
            payload,
        );
        assert_eq!(
            result,
            Err(AdmitGenerateErrorReturn::CompilerVersion {
                text: version.to_string(),
                error: CompilerVersionErrorReturn::Part {
                    index,
                    text: part.to_string()
                }
            })
        );
    }
    // admit_generate_refuses_a_compiler_version_part_that_is_not_a_u16
}

/// Contract: License → `Err(AdmitGenerateErrorReturn::License { error })`.
/// Arrange: `license` `""`.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::License { error:
///   SpdxLicenseIdentifierTryNewErrorReturn::Empty })`.
#[test]
fn admit_generate_refuses_an_empty_license() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    payload.generate.license = String::new();
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::License {
            error: SpdxLicenseIdentifierTryNewErrorReturn::Empty
        })
    );
    // admit_generate_refuses_an_empty_license
}

/// Contract: Ordering → the first refused value in the stated order returns
///   and later values are unread.
/// Arrange: the default payload with `concrete` `"unknown"`, the entry's
///   `count` `Some(0)`, and `license` `""`.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::Concrete { text:
///   "unknown".to_string() })`.
#[test]
fn admit_generate_returns_the_first_refusal_in_the_stated_order() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    payload.generate.concrete = "unknown".to_string();
    payload
        .generate
        .kinds
        .values_mut()
        .next()
        .expect("one entry")
        .count = Some(0);
    payload.generate.license = String::new();
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::Concrete {
            text: "unknown".to_string()
        })
    );
    // admit_generate_returns_the_first_refusal_in_the_stated_order
}

/// Contract: Curve libraries → `Err(AdmitGenerateErrorReturn::
///   CurveLibrariesKey { text })`, a key equal to a member's `Debug` text
///   whole and case-sensitively and never the mock variant's.
/// Arrange: the default payload with the first `curve_libraries` entry keyed
///   `"unknown"`.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::
///   CurveLibrariesKey { text: "unknown".to_string() })`.
#[test]
fn admit_generate_refuses_an_unknown_curve_libraries_key() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    let entry = payload
        .generate
        .curve_libraries
        .values_mut()
        .next()
        .expect("an entry")
        .clone();
    payload.generate.curve_libraries = BTreeMap::from([
        ("unknown".to_string(), entry.clone()),
        (declared_name(&PairingCurve::Bn254), entry),
    ]);
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::CurveLibrariesKey {
            text: "unknown".to_string()
        })
    );
    // admit_generate_refuses_an_unknown_curve_libraries_key
}

/// Contract: the naming rule → a curve libraries key equals a member's
///   `Debug` text whole and case-sensitively.
/// Arrange: the default payload with the `PairingCurve::Bn254` entry keyed by
///   the lowercase of its `declared_name`.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::
///   CurveLibrariesKey { text })` with `text` the lowercase text.
#[test]
fn admit_generate_refuses_a_curve_libraries_key_that_differs_only_in_case() {
    // Arrange
    let text = declared_name(&PairingCurve::Bn254).to_lowercase();
    let mut payload = build_admit_generate_payload(Default::default());
    let entry = payload
        .generate
        .curve_libraries
        .values_mut()
        .next()
        .expect("an entry")
        .clone();
    payload.generate.curve_libraries = BTreeMap::from([
        (declared_name(&PairingCurve::Bls12381), entry.clone()),
        (text.clone(), entry),
    ]);
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(result, Err(AdmitGenerateErrorReturn::CurveLibrariesKey { text }));
    // admit_generate_refuses_a_curve_libraries_key_that_differs_only_in_case
}

/// Contract: Curve libraries → the pairing family's mock variant is never a
///   member of the declared set.
/// Arrange: the default payload with the first `curve_libraries` entry keyed
///   `declared_name(&PairingCurve::Mock)`.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::
///   CurveLibrariesKey { text })` with `text` that name.
#[test]
fn admit_generate_refuses_the_mock_curves_name_as_a_curve_libraries_key() {
    // Arrange
    let text = declared_name(&PairingCurve::Mock);
    let mut payload = build_admit_generate_payload(Default::default());
    let entry = payload
        .generate
        .curve_libraries
        .values_mut()
        .next()
        .expect("an entry")
        .clone();
    payload.generate.curve_libraries = BTreeMap::from([
        (declared_name(&PairingCurve::Bls12381), entry.clone()),
        (text.clone(), entry),
    ]);
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(result, Err(AdmitGenerateErrorReturn::CurveLibrariesKey { text }));
    // admit_generate_refuses_the_mock_curves_name_as_a_curve_libraries_key
}

/// Contract: Missing curve libraries → `Err(AdmitGenerateErrorReturn::
///   MissingCurveLibraries { curve })`, the first declared curve with no
///   entry.
/// Arrange: `curve_libraries` holding the `PairingCurve::Bls12381` entry
///   alone.
/// Act:     `admit_generate`, once.
/// Assert:  the result equals `Err(AdmitGenerateErrorReturn::
///   MissingCurveLibraries { curve: PairingCurve::Bn254 })`.
#[test]
fn admit_generate_refuses_a_declared_curve_with_no_libraries() {
    // Arrange
    let mut payload = build_admit_generate_payload(Default::default());
    let entry = payload
        .generate
        .curve_libraries
        .values_mut()
        .next()
        .expect("an entry")
        .clone();
    payload.generate.curve_libraries =
        BTreeMap::from([(declared_name(&PairingCurve::Bls12381), entry)]);
    // Act
    let result = admit_generate(
        &build_admit_generate_deps(Default::default()),
        build_admit_generate_params(Default::default()),
        payload,
    );
    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::MissingCurveLibraries {
            curve: PairingCurve::Bn254
        })
    );
    // admit_generate_refuses_a_declared_curve_with_no_libraries
}
