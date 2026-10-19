#![allow(clippy::expect_used)]

use std::path::PathBuf;

use super::super::layers::provides::{
    CompilerVersionDocumentOverrides, CurveDirectoriesDocumentOverrides, GenerateDocumentOverrides,
    LibraryDocumentOverrides, VectorKindDocumentOverrides, build_compiler_version_document,
    build_curve_directories_document, build_generate_document, build_library_document,
    build_vector_kind_document,
};
use super::admit_generate;
use super::interface::*;
use super::mock::*;
use crate::generate::provides::*;

/// Contract: each kind text in the vocabulary maps to its `VectorKind`.
/// Arrange: `generate` `build_generate_document` with `kinds` one
///   `build_vector_kind_document` per vocabulary text, each overriding only
///   `kind`, over `"g1_addition"`, `"g1_multiplication"`, `"pairing_check"`,
///   `"g2_addition"`, `"g1_multi_scalar_multiplication"`,
///   `"g2_multi_scalar_multiplication"`, `"decode_rejection"`,
///   `"hash_to_scalar"`, `"identity_mapping"`, `"possession"`, `"challenge"`,
///   `"mint_proof"`, and `"transfer_proof"`, in that order.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the projection of `settings.kinds` to each entry's `kind` equals
///   `Ok(vec![VectorKind::G1Addition, VectorKind::G1Multiplication,
///   VectorKind::PairingCheck, VectorKind::G2Addition,
///   VectorKind::G1MultiScalarMultiplication,
///   VectorKind::G2MultiScalarMultiplication, VectorKind::DecodeRejection,
///   VectorKind::HashToScalar, VectorKind::IdentityMapping,
///   VectorKind::Possession, VectorKind::Challenge, VectorKind::MintProof,
///   VectorKind::TransferProof])`.
#[test]
fn admit_generate_maps_every_vector_kind_text() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(
            [
                "g1_addition",
                "g1_multiplication",
                "pairing_check",
                "g2_addition",
                "g1_multi_scalar_multiplication",
                "g2_multi_scalar_multiplication",
                "decode_rejection",
                "hash_to_scalar",
                "identity_mapping",
                "possession",
                "challenge",
                "mint_proof",
                "transfer_proof",
            ]
            .iter()
            .map(|kind| {
                build_vector_kind_document(VectorKindDocumentOverrides {
                    kind: Some(kind.to_string()),
                    ..Default::default()
                })
            })
            .collect(),
        ),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            let GenerateConcrete::Evm { settings } = success.concrete else {
                panic!("the concrete is the EVM concrete");
            };
            settings
                .kinds
                .iter()
                .map(|kind| kind.kind)
                .collect::<Vec<_>>()
        }),
        Ok(vec![
            VectorKind::G1Addition,
            VectorKind::G1Multiplication,
            VectorKind::PairingCheck,
            VectorKind::G2Addition,
            VectorKind::G1MultiScalarMultiplication,
            VectorKind::G2MultiScalarMultiplication,
            VectorKind::DecodeRejection,
            VectorKind::HashToScalar,
            VectorKind::IdentityMapping,
            VectorKind::Possession,
            VectorKind::Challenge,
            VectorKind::MintProof,
            VectorKind::TransferProof,
        ])
    );
    // admit_generate_maps_every_vector_kind_text
}

/// Contract: the kinds are admitted in the document's order, not the
///   vocabulary's.
/// Arrange: `generate` `build_generate_document` with `kinds` one
///   `build_vector_kind_document` with `kind` `"hash_to_scalar"` then one
///   with `kind` `"g1_addition"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the projection of `settings.kinds` to each entry's `kind` equals
///   `Ok(vec![VectorKind::HashToScalar, VectorKind::G1Addition])`.
#[test]
fn admit_generate_admits_the_kinds_in_document_order() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![
            build_vector_kind_document(VectorKindDocumentOverrides {
                kind: Some("hash_to_scalar".to_string()),
                ..Default::default()
            }),
            build_vector_kind_document(VectorKindDocumentOverrides {
                kind: Some("g1_addition".to_string()),
                ..Default::default()
            }),
        ]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            let GenerateConcrete::Evm { settings } = success.concrete else {
                panic!("the concrete is the EVM concrete");
            };
            settings
                .kinds
                .iter()
                .map(|kind| kind.kind)
                .collect::<Vec<_>>()
        }),
        Ok(vec![VectorKind::HashToScalar, VectorKind::G1Addition])
    );
    // admit_generate_admits_the_kinds_in_document_order
}

/// Contract: a document naming no kind admits no kind and refuses nothing.
/// Arrange: `generate` `build_generate_document` with `kinds` `Vec::new()`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the projection of `settings.kinds` equals `Ok(Vec::new())` and
///   the projection of the success's `kinds` equals `Ok(Vec::new())`.
#[test]
fn admit_generate_admits_an_empty_kind_list() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(Vec::new()),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            let GenerateConcrete::Evm { settings } = success.concrete else {
                panic!("the concrete is the EVM concrete");
            };
            (settings.kinds, success.kinds)
        }),
        Ok((Vec::new(), Vec::new()))
    );
    // admit_generate_admits_an_empty_kind_list
}

/// Contract: a kind named twice is admitted twice, no kind compared with
///   another.
/// Arrange: `generate` `build_generate_document` with `kinds` two
///   `build_vector_kind_document` each with `kind` `"g1_addition"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the projection of `settings.kinds` to each entry's `kind` equals
///   `Ok(vec![VectorKind::G1Addition, VectorKind::G1Addition])`.
#[test]
fn admit_generate_admits_a_kind_named_twice() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![
            build_vector_kind_document(VectorKindDocumentOverrides {
                kind: Some("g1_addition".to_string()),
                ..Default::default()
            }),
            build_vector_kind_document(VectorKindDocumentOverrides {
                kind: Some("g1_addition".to_string()),
                ..Default::default()
            }),
        ]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            let GenerateConcrete::Evm { settings } = success.concrete else {
                panic!("the concrete is the EVM concrete");
            };
            settings
                .kinds
                .iter()
                .map(|kind| kind.kind)
                .collect::<Vec<_>>()
        }),
        Ok(vec![VectorKind::G1Addition, VectorKind::G1Addition])
    );
    // admit_generate_admits_a_kind_named_twice
}

/// Contract: a quantity is admitted whatever the kind, none compared with a
///   declaration.
/// Arrange: `generate` `build_generate_document` with `kinds` holding one
///   `build_vector_kind_document` with `kind` `"decode_rejection"`, `count`
///   `Some(Some(5))`, `terms` `Some(Some(4))`, and `message_length`
///   `Some(Some(64))`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the projection of the success's `kinds` to each entry's
///   `quantities` equals `Ok(vec![build_vector_kind_quantities` with `count`
///   `Some(Some(build_vector_count` over `5))`, `terms`
///   `Some(Some(build_msm_term_count` over `4))`, and `message_length`
///   `Some(Some(64))`])`.
#[test]
fn admit_generate_admits_a_quantity_its_kind_does_not_take() {
    // Arrange
    let expected = build_vector_kind_quantities(VectorKindQuantitiesOverrides {
        count: Some(Some(build_vector_count(
            VectorCountConstructorParamsOverrides { count: Some(5) },
        ))),
        terms: Some(Some(build_msm_term_count(
            MsmTermCountConstructorParamsOverrides { count: Some(4) },
        ))),
        message_length: Some(Some(64)),
    });
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![build_vector_kind_document(
            VectorKindDocumentOverrides {
                kind: Some("decode_rejection".to_string()),
                count: Some(Some(5)),
                terms: Some(Some(4)),
                message_length: Some(Some(64)),
                ..Default::default()
            },
        )]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            success
                .kinds
                .iter()
                .map(|kind| kind.quantities)
                .collect::<Vec<_>>()
        }),
        Ok(vec![expected])
    );
    // admit_generate_admits_a_quantity_its_kind_does_not_take
}

/// Contract: a count the document holds is admitted into
///   `Some(VectorCount)` of it.
/// Arrange: `generate` `build_generate_document` with `kinds` holding one
///   `build_vector_kind_document` with `count` `Some(Some(3))`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the projection of the success's `kinds` to each entry's
///   `quantities` equals `Ok(vec![build_vector_kind_quantities` with `count`
///   `Some(Some(build_vector_count` over `3))`, `terms` `Some(None)`, and
///   `message_length` `Some(None)`])`.
#[test]
fn admit_generate_admits_a_present_count() {
    // Arrange
    let expected = build_vector_kind_quantities(VectorKindQuantitiesOverrides {
        count: Some(Some(build_vector_count(
            VectorCountConstructorParamsOverrides { count: Some(3) },
        ))),
        terms: Some(None),
        message_length: Some(None),
    });
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![build_vector_kind_document(
            VectorKindDocumentOverrides {
                count: Some(Some(3)),
                ..Default::default()
            },
        )]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            success
                .kinds
                .iter()
                .map(|kind| kind.quantities)
                .collect::<Vec<_>>()
        }),
        Ok(vec![expected])
    );
    // admit_generate_admits_a_present_count
}

/// Contract: a count the document does not hold is carried as `None`.
/// Arrange: `generate` `build_generate_document` with `kinds` holding one
///   `build_vector_kind_document` with `count` `Some(None)`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the projection of the success's `kinds` to each entry's
///   `quantities` equals `Ok(vec![build_vector_kind_quantities` with `count`
///   `Some(None)`, `terms` `Some(None)`, and `message_length`
///   `Some(None)`])`.
#[test]
fn admit_generate_carries_an_absent_count_as_absent() {
    // Arrange
    let expected = build_vector_kind_quantities(VectorKindQuantitiesOverrides {
        count: Some(None),
        terms: Some(None),
        message_length: Some(None),
    });
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![build_vector_kind_document(
            VectorKindDocumentOverrides {
                count: Some(None),
                ..Default::default()
            },
        )]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            success
                .kinds
                .iter()
                .map(|kind| kind.quantities)
                .collect::<Vec<_>>()
        }),
        Ok(vec![expected])
    );
    // admit_generate_carries_an_absent_count_as_absent
}

/// Contract: a term count the document holds is admitted into
///   `Some(MsmTermCount)` of it.
/// Arrange: `generate` `build_generate_document` with `kinds` holding one
///   `build_vector_kind_document` with `kind`
///   `"g1_multi_scalar_multiplication"`, `count` `Some(Some(3))`, and `terms`
///   `Some(Some(4))`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the projection of the success's `kinds` to each entry's
///   `quantities` equals `Ok(vec![build_vector_kind_quantities` with `count`
///   `Some(Some(build_vector_count` over `3))`, `terms`
///   `Some(Some(build_msm_term_count` over `4))`, and `message_length`
///   `Some(None)`])`.
#[test]
fn admit_generate_admits_a_present_term_count() {
    // Arrange
    let expected = build_vector_kind_quantities(VectorKindQuantitiesOverrides {
        count: Some(Some(build_vector_count(
            VectorCountConstructorParamsOverrides { count: Some(3) },
        ))),
        terms: Some(Some(build_msm_term_count(
            MsmTermCountConstructorParamsOverrides { count: Some(4) },
        ))),
        message_length: Some(None),
    });
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![build_vector_kind_document(
            VectorKindDocumentOverrides {
                kind: Some("g1_multi_scalar_multiplication".to_string()),
                count: Some(Some(3)),
                terms: Some(Some(4)),
                ..Default::default()
            },
        )]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            success
                .kinds
                .iter()
                .map(|kind| kind.quantities)
                .collect::<Vec<_>>()
        }),
        Ok(vec![expected])
    );
    // admit_generate_admits_a_present_term_count
}

/// Contract: a term count the document does not hold is carried as `None`.
/// Arrange: `generate` `build_generate_document` with `kinds` holding one
///   `build_vector_kind_document` with `kind`
///   `"g1_multi_scalar_multiplication"`, `count` `Some(Some(3))`, and `terms`
///   `Some(None)`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the projection of the success's `kinds` to each entry's
///   `quantities` equals `Ok(vec![build_vector_kind_quantities` with `count`
///   `Some(Some(build_vector_count` over `3))`, `terms` `Some(None)`, and
///   `message_length` `Some(None)`])`.
#[test]
fn admit_generate_carries_an_absent_term_count_as_absent() {
    // Arrange
    let expected = build_vector_kind_quantities(VectorKindQuantitiesOverrides {
        count: Some(Some(build_vector_count(
            VectorCountConstructorParamsOverrides { count: Some(3) },
        ))),
        terms: Some(None),
        message_length: Some(None),
    });
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![build_vector_kind_document(
            VectorKindDocumentOverrides {
                kind: Some("g1_multi_scalar_multiplication".to_string()),
                count: Some(Some(3)),
                terms: Some(None),
                ..Default::default()
            },
        )]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            success
                .kinds
                .iter()
                .map(|kind| kind.quantities)
                .collect::<Vec<_>>()
        }),
        Ok(vec![expected])
    );
    // admit_generate_carries_an_absent_term_count_as_absent
}

/// Contract: a message length the document holds is carried as it is.
/// Arrange: `generate` `build_generate_document` with `kinds` holding one
///   `build_vector_kind_document` with `kind` `"hash_to_scalar"`, `count`
///   `Some(Some(3))`, and `message_length` `Some(Some(64))`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the projection of the success's `kinds` to each entry's
///   `quantities` equals `Ok(vec![build_vector_kind_quantities` with `count`
///   `Some(Some(build_vector_count` over `3))`, `terms` `Some(None)`, and
///   `message_length` `Some(Some(64))`])`.
#[test]
fn admit_generate_carries_a_present_message_length() {
    // Arrange
    let expected = build_vector_kind_quantities(VectorKindQuantitiesOverrides {
        count: Some(Some(build_vector_count(
            VectorCountConstructorParamsOverrides { count: Some(3) },
        ))),
        terms: Some(None),
        message_length: Some(Some(64)),
    });
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![build_vector_kind_document(
            VectorKindDocumentOverrides {
                kind: Some("hash_to_scalar".to_string()),
                count: Some(Some(3)),
                message_length: Some(Some(64)),
                ..Default::default()
            },
        )]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            success
                .kinds
                .iter()
                .map(|kind| kind.quantities)
                .collect::<Vec<_>>()
        }),
        Ok(vec![expected])
    );
    // admit_generate_carries_a_present_message_length
}

/// Contract: a message length the document does not hold is carried as
///   absent.
/// Arrange: `generate` `build_generate_document` with `kinds` holding one
///   `build_vector_kind_document` with `kind` `"hash_to_scalar"`, `count`
///   `Some(Some(3))`, and `message_length` `Some(None)`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the projection of the success's `kinds` to each entry's
///   `quantities` equals `Ok(vec![build_vector_kind_quantities` with `count`
///   `Some(Some(build_vector_count` over `3))`, `terms` `Some(None)`, and
///   `message_length` `Some(None)`])`.
#[test]
fn admit_generate_carries_an_absent_message_length() {
    // Arrange
    let expected = build_vector_kind_quantities(VectorKindQuantitiesOverrides {
        count: Some(Some(build_vector_count(
            VectorCountConstructorParamsOverrides { count: Some(3) },
        ))),
        terms: Some(None),
        message_length: Some(None),
    });
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![build_vector_kind_document(
            VectorKindDocumentOverrides {
                kind: Some("hash_to_scalar".to_string()),
                count: Some(Some(3)),
                message_length: Some(None),
                ..Default::default()
            },
        )]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            success
                .kinds
                .iter()
                .map(|kind| kind.quantities)
                .collect::<Vec<_>>()
        }),
        Ok(vec![expected])
    );
    // admit_generate_carries_an_absent_message_length
}

/// Contract: a kind's file name text is admitted into `GeneratedFileName` in
///   its `names.file_name`.
/// Arrange: `generate` `build_generate_document` with `kinds` holding one
///   `build_vector_kind_document` with `file_name` `"pairing.vectors"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the projection of `settings.kinds` to each entry's
///   `names.file_name` equals `Ok(vec![build_generated_file_name` over
///   `"pairing.vectors"`])`.
#[test]
fn admit_generate_admits_the_kind_file_name() {
    // Arrange
    let expected = build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
        text: Some("pairing.vectors".to_string()),
    });
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![build_vector_kind_document(
            VectorKindDocumentOverrides {
                file_name: Some("pairing.vectors".to_string()),
                ..Default::default()
            },
        )]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            let GenerateConcrete::Evm { settings } = success.concrete else {
                panic!("the concrete is the EVM concrete");
            };
            settings
                .kinds
                .iter()
                .map(|kind| kind.names.file_name.clone())
                .collect::<Vec<_>>()
        }),
        Ok(vec![expected])
    );
    // admit_generate_admits_the_kind_file_name
}

/// Contract: a kind's struct name text is admitted into `SolidityTypeName`
///   in its `names.struct_name`.
/// Arrange: `generate` `build_generate_document` with `kinds` holding one
///   `build_vector_kind_document` with `struct_name` `"PairingVector"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the projection of `settings.kinds` to each entry's
///   `names.struct_name` equals `Ok(vec![build_solidity_type_name` over
///   `"PairingVector"`])`.
#[test]
fn admit_generate_admits_the_kind_struct_name() {
    // Arrange
    let expected = build_solidity_type_name(SolidityTypeNameConstructorParamsOverrides {
        text: Some("PairingVector".to_string()),
    });
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![build_vector_kind_document(
            VectorKindDocumentOverrides {
                struct_name: Some("PairingVector".to_string()),
                ..Default::default()
            },
        )]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            let GenerateConcrete::Evm { settings } = success.concrete else {
                panic!("the concrete is the EVM concrete");
            };
            settings
                .kinds
                .iter()
                .map(|kind| kind.names.struct_name.clone())
                .collect::<Vec<_>>()
        }),
        Ok(vec![expected])
    );
    // admit_generate_admits_the_kind_struct_name
}

/// Contract: a kind's decode function name text is admitted into
///   `SolidityMemberName` in its `names.decode_function_name`.
/// Arrange: `generate` `build_generate_document` with `kinds` holding one
///   `build_vector_kind_document` with `decode_function_name`
///   `"decodePairing"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the projection of `settings.kinds` to each entry's
///   `names.decode_function_name` equals
///   `Ok(vec![build_solidity_member_name` over `"decodePairing"`])`.
#[test]
fn admit_generate_admits_the_kind_decode_function_name() {
    // Arrange
    let expected = build_solidity_member_name(SolidityMemberNameConstructorParamsOverrides {
        text: Some("decodePairing".to_string()),
    });
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![build_vector_kind_document(
            VectorKindDocumentOverrides {
                decode_function_name: Some("decodePairing".to_string()),
                ..Default::default()
            },
        )]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            let GenerateConcrete::Evm { settings } = success.concrete else {
                panic!("the concrete is the EVM concrete");
            };
            settings
                .kinds
                .iter()
                .map(|kind| kind.names.decode_function_name.clone())
                .collect::<Vec<_>>()
        }),
        Ok(vec![expected])
    );
    // admit_generate_admits_the_kind_decode_function_name
}

/// Contract: a kind's path constant name text is admitted into
///   `SolidityConstantName` in its `names.path_constant_name`.
/// Arrange: `generate` `build_generate_document` with `kinds` holding one
///   `build_vector_kind_document` with `path_constant_name`
///   `"PAIRING_PATH"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the projection of `settings.kinds` to each entry's
///   `names.path_constant_name` equals
///   `Ok(vec![build_solidity_constant_name` over `"PAIRING_PATH"`])`.
#[test]
fn admit_generate_admits_the_kind_path_constant_name() {
    // Arrange
    let expected = build_solidity_constant_name(SolidityConstantNameConstructorParamsOverrides {
        text: Some("PAIRING_PATH".to_string()),
    });
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![build_vector_kind_document(
            VectorKindDocumentOverrides {
                path_constant_name: Some("PAIRING_PATH".to_string()),
                ..Default::default()
            },
        )]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            let GenerateConcrete::Evm { settings } = success.concrete else {
                panic!("the concrete is the EVM concrete");
            };
            settings
                .kinds
                .iter()
                .map(|kind| kind.names.path_constant_name.clone())
                .collect::<Vec<_>>()
        }),
        Ok(vec![expected])
    );
    // admit_generate_admits_the_kind_path_constant_name
}

/// Contract: the constants library's file name text is admitted into
///   `GeneratedFileName` in `constants_library.file_name`.
/// Arrange: `generate` `build_generate_document` with `constants_library`
///   `build_library_document` with `file_name` `"Pairing.sol"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: `settings.constants_library.file_name` equals
///   `Ok(build_generated_file_name` over `"Pairing.sol")`.
#[test]
fn admit_generate_admits_the_constants_library_file_name() {
    // Arrange
    let expected = build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
        text: Some("Pairing.sol".to_string()),
    });
    let generate = build_generate_document(GenerateDocumentOverrides {
        constants_library: Some(build_library_document(LibraryDocumentOverrides {
            file_name: Some("Pairing.sol".to_string()),
            ..Default::default()
        })),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            let GenerateConcrete::Evm { settings } = success.concrete else {
                panic!("the concrete is the EVM concrete");
            };
            settings.constants_library.file_name
        }),
        Ok(expected)
    );
    // admit_generate_admits_the_constants_library_file_name
}

/// Contract: the constants library's name text is admitted into
///   `SolidityLibraryName` in `constants_library.library_name`.
/// Arrange: `generate` `build_generate_document` with `constants_library`
///   `build_library_document` with `library_name` `"PairingConstants"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: `settings.constants_library.library_name` equals
///   `Ok(build_solidity_library_name` over `"PairingConstants")`.
#[test]
fn admit_generate_admits_the_constants_library_name() {
    // Arrange
    let expected = build_solidity_library_name(SolidityLibraryNameConstructorParamsOverrides {
        text: Some("PairingConstants".to_string()),
    });
    let generate = build_generate_document(GenerateDocumentOverrides {
        constants_library: Some(build_library_document(LibraryDocumentOverrides {
            library_name: Some("PairingConstants".to_string()),
            ..Default::default()
        })),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            let GenerateConcrete::Evm { settings } = success.concrete else {
                panic!("the concrete is the EVM concrete");
            };
            settings.constants_library.library_name
        }),
        Ok(expected)
    );
    // admit_generate_admits_the_constants_library_name
}

/// Contract: the records library's file name text is admitted into
///   `GeneratedFileName` in `records_library.file_name`.
/// Arrange: `generate` `build_generate_document` with `records_library`
///   `build_library_document` with `file_name` `"Records.sol"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: `settings.records_library.file_name` equals
///   `Ok(build_generated_file_name` over `"Records.sol")`.
#[test]
fn admit_generate_admits_the_records_library_file_name() {
    // Arrange
    let expected = build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
        text: Some("Records.sol".to_string()),
    });
    let generate = build_generate_document(GenerateDocumentOverrides {
        records_library: Some(build_library_document(LibraryDocumentOverrides {
            file_name: Some("Records.sol".to_string()),
            ..Default::default()
        })),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            let GenerateConcrete::Evm { settings } = success.concrete else {
                panic!("the concrete is the EVM concrete");
            };
            settings.records_library.file_name
        }),
        Ok(expected)
    );
    // admit_generate_admits_the_records_library_file_name
}

/// Contract: the records library's name text is admitted into
///   `SolidityLibraryName` in `records_library.library_name`.
/// Arrange: `generate` `build_generate_document` with `records_library`
///   `build_library_document` with `library_name` `"Records"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: `settings.records_library.library_name` equals
///   `Ok(build_solidity_library_name` over `"Records")`.
#[test]
fn admit_generate_admits_the_records_library_name() {
    // Arrange
    let expected = build_solidity_library_name(SolidityLibraryNameConstructorParamsOverrides {
        text: Some("Records".to_string()),
    });
    let generate = build_generate_document(GenerateDocumentOverrides {
        records_library: Some(build_library_document(LibraryDocumentOverrides {
            library_name: Some("Records".to_string()),
            ..Default::default()
        })),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            let GenerateConcrete::Evm { settings } = success.concrete else {
                panic!("the concrete is the EVM concrete");
            };
            settings.records_library.library_name
        }),
        Ok(expected)
    );
    // admit_generate_admits_the_records_library_name
}

/// Contract: the Foundry project root is carried as the document holds it.
/// Arrange: `generate` `build_generate_document` with `foundry_root`
///   `PathBuf::from("suite")`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: `settings.foundry_root` equals `Ok(PathBuf::from("suite"))`.
#[test]
fn admit_generate_carries_the_foundry_root() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        foundry_root: Some(PathBuf::from("suite")),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            let GenerateConcrete::Evm { settings } = success.concrete else {
                panic!("the concrete is the EVM concrete");
            };
            settings.foundry_root
        }),
        Ok(PathBuf::from("suite"))
    );
    // admit_generate_carries_the_foundry_root
}

/// Contract: the generated directory's segments are admitted in order into
///   one `GeneratedDirectory`.
/// Arrange: `generate` `build_generate_document` with `generated_directory`
///   `vec!["out".to_string(), "vectors".to_string()]`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: `settings.generated_directory` equals
///   `Ok(build_generated_directory` over `build_generated_file_name` of
///   `"out"` then of `"vectors")`.
#[test]
fn admit_generate_admits_the_generated_directory_segments_in_order() {
    // Arrange
    let expected = build_generated_directory(GeneratedDirectoryConstructorParamsOverrides {
        segments: Some(vec![
            build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
                text: Some("out".to_string()),
            }),
            build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
                text: Some("vectors".to_string()),
            }),
        ]),
    });
    let generate = build_generate_document(GenerateDocumentOverrides {
        generated_directory: Some(vec!["out".to_string(), "vectors".to_string()]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            let GenerateConcrete::Evm { settings } = success.concrete else {
                panic!("the concrete is the EVM concrete");
            };
            settings.generated_directory
        }),
        Ok(expected)
    );
    // admit_generate_admits_the_generated_directory_segments_in_order
}

/// Contract: the bn254 directory text is admitted into `GeneratedFileName`
///   in `curve_directories.bn254`.
/// Arrange: `generate` `build_generate_document` with `curve_directories`
///   `build_curve_directories_document` with `bn254` `"alt_bn128"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: `settings.curve_directories.bn254` equals
///   `Ok(build_generated_file_name` over `"alt_bn128")`.
#[test]
fn admit_generate_admits_the_bn254_directory() {
    // Arrange
    let expected = build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
        text: Some("alt_bn128".to_string()),
    });
    let generate = build_generate_document(GenerateDocumentOverrides {
        curve_directories: Some(build_curve_directories_document(
            CurveDirectoriesDocumentOverrides {
                bn254: Some("alt_bn128".to_string()),
                ..Default::default()
            },
        )),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            let GenerateConcrete::Evm { settings } = success.concrete else {
                panic!("the concrete is the EVM concrete");
            };
            settings.curve_directories.bn254
        }),
        Ok(expected)
    );
    // admit_generate_admits_the_bn254_directory
}

/// Contract: the bls12_381 directory text is admitted into
///   `GeneratedFileName` in `curve_directories.bls12_381`.
/// Arrange: `generate` `build_generate_document` with `curve_directories`
///   `build_curve_directories_document` with `bls12_381` `"bls"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: `settings.curve_directories.bls12_381` equals
///   `Ok(build_generated_file_name` over `"bls")`.
#[test]
fn admit_generate_admits_the_bls12_381_directory() {
    // Arrange
    let expected = build_generated_file_name(GeneratedFileNameConstructorParamsOverrides {
        text: Some("bls".to_string()),
    });
    let generate = build_generate_document(GenerateDocumentOverrides {
        curve_directories: Some(build_curve_directories_document(
            CurveDirectoriesDocumentOverrides {
                bls12_381: Some("bls".to_string()),
                ..Default::default()
            },
        )),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            let GenerateConcrete::Evm { settings } = success.concrete else {
                panic!("the concrete is the EVM concrete");
            };
            settings.curve_directories.bls12_381
        }),
        Ok(expected)
    );
    // admit_generate_admits_the_bls12_381_directory
}

/// Contract: each field of the document's compiler version is carried into
///   the field of its name.
/// Arrange: `generate` `build_generate_document` with `compiler_version`
///   `build_compiler_version_document` over `major` `2`, `minor` `7`, and
///   `patch` `30`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: `settings.compiler_version` equals
///   `Ok(build_solidity_compiler_version` over `2`, `7`, and `30)`.
#[test]
fn admit_generate_carries_the_compiler_version() {
    // Arrange
    let expected = build_solidity_compiler_version(SolidityCompilerVersionOverrides {
        major: Some(2),
        minor: Some(7),
        patch: Some(30),
    });
    let generate = build_generate_document(GenerateDocumentOverrides {
        compiler_version: Some(build_compiler_version_document(
            CompilerVersionDocumentOverrides {
                major: Some(2),
                minor: Some(7),
                patch: Some(30),
            },
        )),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            let GenerateConcrete::Evm { settings } = success.concrete else {
                panic!("the concrete is the EVM concrete");
            };
            settings.compiler_version
        }),
        Ok(expected)
    );
    // admit_generate_carries_the_compiler_version
}

/// Contract: the license text is admitted into `SpdxLicenseIdentifier`.
/// Arrange: `generate` `build_generate_document` with `license`
///   `"Apache-2.0"`, a text neither the layers builder's nor the render
///   builder's default.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: `settings.license` equals `Ok(build_spdx_license_identifier` over
///   `"Apache-2.0")`.
#[test]
fn admit_generate_admits_the_license() {
    // Arrange
    let expected = build_spdx_license_identifier(SpdxLicenseIdentifierConstructorParamsOverrides {
        text: Some("Apache-2.0".to_string()),
    });
    let generate = build_generate_document(GenerateDocumentOverrides {
        license: Some("Apache-2.0".to_string()),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result.map(|success| {
            let GenerateConcrete::Evm { settings } = success.concrete else {
                panic!("the concrete is the EVM concrete");
            };
            settings.license
        }),
        Ok(expected)
    );
    // admit_generate_admits_the_license
}

/// Contract: a concrete text outside the vocabulary is refused with
///   `UnknownValue` naming the concrete field and the text.
/// Arrange: `generate` `build_generate_document` with `concrete`
///   `"solidity"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::UnknownValue
///   { field: GenerateField::Concrete, text: "solidity".to_string() })`.
#[test]
fn admit_generate_refuses_an_unknown_concrete() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        concrete: Some("solidity".to_string()),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::UnknownValue {
            field: GenerateField::Concrete,
            text: "solidity".to_string(),
        })
    );
    // admit_generate_refuses_an_unknown_concrete
}

/// Contract: a kind text outside the vocabulary is refused with
///   `UnknownValue` naming the kind field at its position and the text.
/// Arrange: `generate` `build_generate_document` with `kinds`
///   `build_vector_kind_document` with no override then one with `kind`
///   `"g1_add"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::UnknownValue
///   { field: GenerateField::Kind { index: 1 }, text:
///   "g1_add".to_string() })`.
#[test]
fn admit_generate_refuses_an_unknown_kind_with_its_index() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![
            build_vector_kind_document(Default::default()),
            build_vector_kind_document(VectorKindDocumentOverrides {
                kind: Some("g1_add".to_string()),
                ..Default::default()
            }),
        ]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::UnknownValue {
            field: GenerateField::Kind { index: 1 },
            text: "g1_add".to_string(),
        })
    );
    // admit_generate_refuses_an_unknown_kind_with_its_index
}

/// Contract: a count of zero is refused with `VectorCount` carrying the
///   kind's position and the constructor's refusal.
/// Arrange: `generate` `build_generate_document` with `kinds`
///   `build_vector_kind_document` with no override then one with `count`
///   `Some(Some(0))`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::VectorCount
///   { index: 1, error: VectorCountTryNewErrorReturn::Zero })`.
#[test]
fn admit_generate_refuses_a_zero_count_with_its_index() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![
            build_vector_kind_document(Default::default()),
            build_vector_kind_document(VectorKindDocumentOverrides {
                count: Some(Some(0)),
                ..Default::default()
            }),
        ]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::VectorCount {
            index: 1,
            error: VectorCountTryNewErrorReturn::Zero,
        })
    );
    // admit_generate_refuses_a_zero_count_with_its_index
}

/// Contract: a term count of zero is refused with `MsmTermCount` carrying
///   the kind's position and the constructor's refusal.
/// Arrange: `generate` `build_generate_document` with `kinds`
///   `build_vector_kind_document` with no override then one with `kind`
///   `"g1_multi_scalar_multiplication"` and `terms` `Some(Some(0))`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::MsmTermCount
///   { index: 1, error: MsmTermCountTryNewErrorReturn::Zero })`.
#[test]
fn admit_generate_refuses_a_zero_term_count_with_its_index() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![
            build_vector_kind_document(Default::default()),
            build_vector_kind_document(VectorKindDocumentOverrides {
                kind: Some("g1_multi_scalar_multiplication".to_string()),
                terms: Some(Some(0)),
                ..Default::default()
            }),
        ]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::MsmTermCount {
            index: 1,
            error: MsmTermCountTryNewErrorReturn::Zero,
        })
    );
    // admit_generate_refuses_a_zero_term_count_with_its_index
}

/// Contract: a file name text the constructor refuses is refused with
///   `FileName` naming the kind's file name field at its position.
/// Arrange: `generate` `build_generate_document` with `kinds`
///   `build_vector_kind_document` with no override then one with
///   `file_name` `"a/b.vectors"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::FileName
///   { field: GenerateField::KindFileName { index: 1 }, error:
///   GeneratedFileNameTryNewErrorReturn::InvalidCharacter { index: 1, byte:
///   b'/' } })`.
#[test]
fn admit_generate_refuses_a_kind_file_name_holding_a_separator_with_its_index() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![
            build_vector_kind_document(Default::default()),
            build_vector_kind_document(VectorKindDocumentOverrides {
                file_name: Some("a/b.vectors".to_string()),
                ..Default::default()
            }),
        ]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::FileName {
            field: GenerateField::KindFileName { index: 1 },
            error: GeneratedFileNameTryNewErrorReturn::InvalidCharacter {
                index: 1,
                byte: b'/',
            },
        })
    );
    // admit_generate_refuses_a_kind_file_name_holding_a_separator_with_its_index
}

/// Contract: a struct name text the constructor refuses is refused with
///   `StructName` carrying the kind's position and the refusal.
/// Arrange: `generate` `build_generate_document` with `kinds`
///   `build_vector_kind_document` with no override then one with
///   `struct_name` `"g1AddVector"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::StructName
///   { index: 1, error:
///   SolidityTypeNameTryNewErrorReturn::LeadingCharacter { byte: b'g' } })`.
#[test]
fn admit_generate_refuses_a_struct_name_with_a_lowercase_lead_with_its_index() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![
            build_vector_kind_document(Default::default()),
            build_vector_kind_document(VectorKindDocumentOverrides {
                struct_name: Some("g1AddVector".to_string()),
                ..Default::default()
            }),
        ]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::StructName {
            index: 1,
            error: SolidityTypeNameTryNewErrorReturn::LeadingCharacter { byte: b'g' },
        })
    );
    // admit_generate_refuses_a_struct_name_with_a_lowercase_lead_with_its_index
}

/// Contract: a decode function name text the constructor refuses is refused
///   with `DecodeFunctionName` carrying the kind's position and the refusal.
/// Arrange: `generate` `build_generate_document` with `kinds`
///   `build_vector_kind_document` with no override then one with
///   `decode_function_name` `"DecodeG1Add"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::DecodeFunctionName
///   { index: 1, error:
///   SolidityMemberNameTryNewErrorReturn::LeadingCharacter { byte: b'D' } })`.
#[test]
fn admit_generate_refuses_a_decode_function_name_with_an_uppercase_lead_with_its_index() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![
            build_vector_kind_document(Default::default()),
            build_vector_kind_document(VectorKindDocumentOverrides {
                decode_function_name: Some("DecodeG1Add".to_string()),
                ..Default::default()
            }),
        ]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::DecodeFunctionName {
            index: 1,
            error: SolidityMemberNameTryNewErrorReturn::LeadingCharacter { byte: b'D' },
        })
    );
    // admit_generate_refuses_a_decode_function_name_with_an_uppercase_lead_with_its_index
}

/// Contract: a path constant name text the constructor refuses is refused
///   with `PathConstantName` carrying the kind's position and the refusal.
/// Arrange: `generate` `build_generate_document` with `kinds`
///   `build_vector_kind_document` with no override then one with
///   `path_constant_name` `"G1_add_PATH"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::PathConstantName
///   { index: 1, error:
///   SolidityConstantNameTryNewErrorReturn::InvalidCharacter { index: 3,
///   byte: b'a' } })`.
#[test]
fn admit_generate_refuses_a_path_constant_name_holding_a_lowercase_letter_with_its_index() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![
            build_vector_kind_document(Default::default()),
            build_vector_kind_document(VectorKindDocumentOverrides {
                path_constant_name: Some("G1_add_PATH".to_string()),
                ..Default::default()
            }),
        ]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::PathConstantName {
            index: 1,
            error: SolidityConstantNameTryNewErrorReturn::InvalidCharacter {
                index: 3,
                byte: b'a',
            },
        })
    );
    // admit_generate_refuses_a_path_constant_name_holding_a_lowercase_letter_with_its_index
}

/// Contract: the constants library's file name text the constructor refuses
///   is refused with `FileName` naming the constants library's file name
///   field.
/// Arrange: `generate` `build_generate_document` with `constants_library`
///   `build_library_document` with `file_name` `""`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::FileName
///   { field: GenerateField::ConstantsLibraryFileName, error:
///   GeneratedFileNameTryNewErrorReturn::Empty })`.
#[test]
fn admit_generate_refuses_an_empty_constants_library_file_name() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        constants_library: Some(build_library_document(LibraryDocumentOverrides {
            file_name: Some(String::new()),
            ..Default::default()
        })),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::FileName {
            field: GenerateField::ConstantsLibraryFileName,
            error: GeneratedFileNameTryNewErrorReturn::Empty,
        })
    );
    // admit_generate_refuses_an_empty_constants_library_file_name
}

/// Contract: the constants library's name text the constructor refuses is
///   refused with `LibraryName` naming the constants library's name field.
/// Arrange: `generate` `build_generate_document` with `constants_library`
///   `build_library_document` with `library_name` `"Vector_Records"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::LibraryName
///   { field: GenerateField::ConstantsLibraryName, error:
///   SolidityLibraryNameTryNewErrorReturn::InvalidCharacter { index: 6,
///   byte: b'_' } })`.
#[test]
fn admit_generate_refuses_a_constants_library_name_holding_an_invalid_character() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        constants_library: Some(build_library_document(LibraryDocumentOverrides {
            library_name: Some("Vector_Records".to_string()),
            ..Default::default()
        })),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::LibraryName {
            field: GenerateField::ConstantsLibraryName,
            error: SolidityLibraryNameTryNewErrorReturn::InvalidCharacter {
                index: 6,
                byte: b'_',
            },
        })
    );
    // admit_generate_refuses_a_constants_library_name_holding_an_invalid_character
}

/// Contract: the records library's file name text the constructor refuses
///   is refused with `FileName` naming the records library's file name
///   field.
/// Arrange: `generate` `build_generate_document` with `records_library`
///   `build_library_document` with `file_name` `""`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::FileName
///   { field: GenerateField::RecordsLibraryFileName, error:
///   GeneratedFileNameTryNewErrorReturn::Empty })`.
#[test]
fn admit_generate_refuses_an_empty_records_library_file_name() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        records_library: Some(build_library_document(LibraryDocumentOverrides {
            file_name: Some(String::new()),
            ..Default::default()
        })),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::FileName {
            field: GenerateField::RecordsLibraryFileName,
            error: GeneratedFileNameTryNewErrorReturn::Empty,
        })
    );
    // admit_generate_refuses_an_empty_records_library_file_name
}

/// Contract: the records library's name text the constructor refuses is
///   refused with `LibraryName` naming the records library's name field.
/// Arrange: `generate` `build_generate_document` with `records_library`
///   `build_library_document` with `library_name` `"Vector_Records"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::LibraryName
///   { field: GenerateField::RecordsLibraryName, error:
///   SolidityLibraryNameTryNewErrorReturn::InvalidCharacter { index: 6,
///   byte: b'_' } })`.
#[test]
fn admit_generate_refuses_a_records_library_name_holding_an_invalid_character() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        records_library: Some(build_library_document(LibraryDocumentOverrides {
            library_name: Some("Vector_Records".to_string()),
            ..Default::default()
        })),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::LibraryName {
            field: GenerateField::RecordsLibraryName,
            error: SolidityLibraryNameTryNewErrorReturn::InvalidCharacter {
                index: 6,
                byte: b'_',
            },
        })
    );
    // admit_generate_refuses_a_records_library_name_holding_an_invalid_character
}

/// Contract: a segment the constructor refuses is refused with `FileName`
///   naming the segment field at its position.
/// Arrange: `generate` `build_generate_document` with `generated_directory`
///   `vec!["generated".to_string(), "..".to_string()]`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::FileName
///   { field: GenerateField::GeneratedDirectorySegment { index: 1 }, error:
///   GeneratedFileNameTryNewErrorReturn::Reserved })`.
#[test]
fn admit_generate_refuses_a_reserved_directory_segment_with_its_index() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        generated_directory: Some(vec!["generated".to_string(), "..".to_string()]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::FileName {
            field: GenerateField::GeneratedDirectorySegment { index: 1 },
            error: GeneratedFileNameTryNewErrorReturn::Reserved,
        })
    );
    // admit_generate_refuses_a_reserved_directory_segment_with_its_index
}

/// Contract: a document naming no segment is refused with
///   `GeneratedDirectory` carrying the constructor's refusal.
/// Arrange: `generate` `build_generate_document` with `generated_directory`
///   `Vec::new()`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::GeneratedDirectory(
///   GeneratedDirectoryTryNewErrorReturn::Empty))`.
#[test]
fn admit_generate_refuses_an_empty_generated_directory() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        generated_directory: Some(Vec::new()),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::GeneratedDirectory(
            GeneratedDirectoryTryNewErrorReturn::Empty
        ))
    );
    // admit_generate_refuses_an_empty_generated_directory
}

/// Contract: the bn254 directory text the constructor refuses is refused
///   with `FileName` naming the bn254 field.
/// Arrange: `generate` `build_generate_document` with `curve_directories`
///   `build_curve_directories_document` with `bn254` `"bn/254"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::FileName
///   { field: GenerateField::Bn254Directory, error:
///   GeneratedFileNameTryNewErrorReturn::InvalidCharacter { index: 2, byte:
///   b'/' } })`.
#[test]
fn admit_generate_refuses_a_bn254_directory_holding_a_separator() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        curve_directories: Some(build_curve_directories_document(
            CurveDirectoriesDocumentOverrides {
                bn254: Some("bn/254".to_string()),
                ..Default::default()
            },
        )),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::FileName {
            field: GenerateField::Bn254Directory,
            error: GeneratedFileNameTryNewErrorReturn::InvalidCharacter {
                index: 2,
                byte: b'/',
            },
        })
    );
    // admit_generate_refuses_a_bn254_directory_holding_a_separator
}

/// Contract: the bls12_381 directory text the constructor refuses is
///   refused with `FileName` naming the bls12_381 field.
/// Arrange: `generate` `build_generate_document` with `curve_directories`
///   `build_curve_directories_document` with `bls12_381` `"bls/12"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::FileName
///   { field: GenerateField::Bls12381Directory, error:
///   GeneratedFileNameTryNewErrorReturn::InvalidCharacter { index: 3, byte:
///   b'/' } })`.
#[test]
fn admit_generate_refuses_a_bls12_381_directory_holding_a_separator() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        curve_directories: Some(build_curve_directories_document(
            CurveDirectoriesDocumentOverrides {
                bls12_381: Some("bls/12".to_string()),
                ..Default::default()
            },
        )),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::FileName {
            field: GenerateField::Bls12381Directory,
            error: GeneratedFileNameTryNewErrorReturn::InvalidCharacter {
                index: 3,
                byte: b'/',
            },
        })
    );
    // admit_generate_refuses_a_bls12_381_directory_holding_a_separator
}

/// Contract: a license text the constructor refuses is refused with
///   `License` carrying the refusal.
/// Arrange: `generate` `build_generate_document` with `license` `""`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::License(
///   SpdxLicenseIdentifierTryNewErrorReturn::Empty))`.
#[test]
fn admit_generate_refuses_an_empty_license() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        license: Some(String::new()),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::License(
            SpdxLicenseIdentifierTryNewErrorReturn::Empty
        ))
    );
    // admit_generate_refuses_an_empty_license
}

/// Contract: an unknown concrete and an unknown kind in one document are
///   refused with the concrete's refusal.
/// Arrange: `generate` `build_generate_document` with `concrete`
///   `"solidity"` and `kinds` holding one `build_vector_kind_document` with
///   `kind` `"g1_add"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::UnknownValue
///   { field: GenerateField::Concrete, text: "solidity".to_string() })`.
#[test]
fn admit_generate_refuses_the_concrete_before_a_kind() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        concrete: Some("solidity".to_string()),
        kinds: Some(vec![build_vector_kind_document(
            VectorKindDocumentOverrides {
                kind: Some("g1_add".to_string()),
                ..Default::default()
            },
        )]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::UnknownValue {
            field: GenerateField::Concrete,
            text: "solidity".to_string(),
        })
    );
    // admit_generate_refuses_the_concrete_before_a_kind
}

/// Contract: two kinds each refused are refused with the earlier kind's
///   refusal.
/// Arrange: `generate` `build_generate_document` with `kinds` one
///   `build_vector_kind_document` with `kind` `"g1_add"` then one with
///   `kind` `"g2_add"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::UnknownValue
///   { field: GenerateField::Kind { index: 0 }, text:
///   "g1_add".to_string() })`.
#[test]
fn admit_generate_refuses_an_earlier_kind_before_a_later_kind() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![
            build_vector_kind_document(VectorKindDocumentOverrides {
                kind: Some("g1_add".to_string()),
                ..Default::default()
            }),
            build_vector_kind_document(VectorKindDocumentOverrides {
                kind: Some("g2_add".to_string()),
                ..Default::default()
            }),
        ]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::UnknownValue {
            field: GenerateField::Kind { index: 0 },
            text: "g1_add".to_string(),
        })
    );
    // admit_generate_refuses_an_earlier_kind_before_a_later_kind
}

/// Contract: a kind text and a count each refused are refused with the kind
///   text's refusal.
/// Arrange: `generate` `build_generate_document` with `kinds` holding one
///   `build_vector_kind_document` with `kind` `"g1_add"` and `count`
///   `Some(Some(0))`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::UnknownValue
///   { field: GenerateField::Kind { index: 0 }, text:
///   "g1_add".to_string() })`.
#[test]
fn admit_generate_refuses_a_kinds_text_before_its_count() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![build_vector_kind_document(
            VectorKindDocumentOverrides {
                kind: Some("g1_add".to_string()),
                count: Some(Some(0)),
                ..Default::default()
            },
        )]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::UnknownValue {
            field: GenerateField::Kind { index: 0 },
            text: "g1_add".to_string(),
        })
    );
    // admit_generate_refuses_a_kinds_text_before_its_count
}

/// Contract: a count and a term count each refused are refused with the
///   count's refusal.
/// Arrange: `generate` `build_generate_document` with `kinds` holding one
///   `build_vector_kind_document` with `kind`
///   `"g1_multi_scalar_multiplication"`, `count` `Some(Some(0))`, and
///   `terms` `Some(Some(0))`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::VectorCount
///   { index: 0, error: VectorCountTryNewErrorReturn::Zero })`.
#[test]
fn admit_generate_refuses_a_kinds_count_before_its_term_count() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![build_vector_kind_document(
            VectorKindDocumentOverrides {
                kind: Some("g1_multi_scalar_multiplication".to_string()),
                count: Some(Some(0)),
                terms: Some(Some(0)),
                ..Default::default()
            },
        )]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::VectorCount {
            index: 0,
            error: VectorCountTryNewErrorReturn::Zero,
        })
    );
    // admit_generate_refuses_a_kinds_count_before_its_term_count
}

/// Contract: a term count and a file name each refused are refused with the
///   term count's refusal.
/// Arrange: `generate` `build_generate_document` with `kinds` holding one
///   `build_vector_kind_document` with `kind`
///   `"g1_multi_scalar_multiplication"`, `terms` `Some(Some(0))`, and
///   `file_name` `"a/b.vectors"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::MsmTermCount
///   { index: 0, error: MsmTermCountTryNewErrorReturn::Zero })`.
#[test]
fn admit_generate_refuses_a_kinds_term_count_before_its_file_name() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![build_vector_kind_document(
            VectorKindDocumentOverrides {
                kind: Some("g1_multi_scalar_multiplication".to_string()),
                terms: Some(Some(0)),
                file_name: Some("a/b.vectors".to_string()),
                ..Default::default()
            },
        )]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::MsmTermCount {
            index: 0,
            error: MsmTermCountTryNewErrorReturn::Zero,
        })
    );
    // admit_generate_refuses_a_kinds_term_count_before_its_file_name
}

/// Contract: a file name and a struct name each refused are refused with
///   the file name's refusal.
/// Arrange: `generate` `build_generate_document` with `kinds` holding one
///   `build_vector_kind_document` with `file_name` `"a/b.vectors"` and
///   `struct_name` `"g1AddVector"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::FileName
///   { field: GenerateField::KindFileName { index: 0 }, error:
///   GeneratedFileNameTryNewErrorReturn::InvalidCharacter { index: 1, byte:
///   b'/' } })`.
#[test]
fn admit_generate_refuses_a_kinds_file_name_before_its_struct_name() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![build_vector_kind_document(
            VectorKindDocumentOverrides {
                file_name: Some("a/b.vectors".to_string()),
                struct_name: Some("g1AddVector".to_string()),
                ..Default::default()
            },
        )]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::FileName {
            field: GenerateField::KindFileName { index: 0 },
            error: GeneratedFileNameTryNewErrorReturn::InvalidCharacter {
                index: 1,
                byte: b'/',
            },
        })
    );
    // admit_generate_refuses_a_kinds_file_name_before_its_struct_name
}

/// Contract: a struct name and a decode function name each refused are
///   refused with the struct name's refusal.
/// Arrange: `generate` `build_generate_document` with `kinds` holding one
///   `build_vector_kind_document` with `struct_name` `"g1AddVector"` and
///   `decode_function_name` `"DecodeG1Add"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::StructName
///   { index: 0, error:
///   SolidityTypeNameTryNewErrorReturn::LeadingCharacter { byte: b'g' } })`.
#[test]
fn admit_generate_refuses_a_kinds_struct_name_before_its_decode_function_name() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![build_vector_kind_document(
            VectorKindDocumentOverrides {
                struct_name: Some("g1AddVector".to_string()),
                decode_function_name: Some("DecodeG1Add".to_string()),
                ..Default::default()
            },
        )]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::StructName {
            index: 0,
            error: SolidityTypeNameTryNewErrorReturn::LeadingCharacter { byte: b'g' },
        })
    );
    // admit_generate_refuses_a_kinds_struct_name_before_its_decode_function_name
}

/// Contract: a decode function name and a path constant name each refused
///   are refused with the decode function name's refusal.
/// Arrange: `generate` `build_generate_document` with `kinds` holding one
///   `build_vector_kind_document` with `decode_function_name`
///   `"DecodeG1Add"` and `path_constant_name` `"G1_add_PATH"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::DecodeFunctionName
///   { index: 0, error:
///   SolidityMemberNameTryNewErrorReturn::LeadingCharacter { byte: b'D' } })`.
#[test]
fn admit_generate_refuses_a_kinds_decode_function_name_before_its_path_constant_name() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![build_vector_kind_document(
            VectorKindDocumentOverrides {
                decode_function_name: Some("DecodeG1Add".to_string()),
                path_constant_name: Some("G1_add_PATH".to_string()),
                ..Default::default()
            },
        )]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::DecodeFunctionName {
            index: 0,
            error: SolidityMemberNameTryNewErrorReturn::LeadingCharacter { byte: b'D' },
        })
    );
    // admit_generate_refuses_a_kinds_decode_function_name_before_its_path_constant_name
}

/// Contract: a kind and the constants library each refused are refused with
///   the kind's refusal.
/// Arrange: `generate` `build_generate_document` with `kinds` holding one
///   `build_vector_kind_document` with `path_constant_name`
///   `"G1_add_PATH"` and `constants_library` `build_library_document` with
///   `file_name` `""`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::PathConstantName
///   { index: 0, error:
///   SolidityConstantNameTryNewErrorReturn::InvalidCharacter { index: 3,
///   byte: b'a' } })`.
#[test]
fn admit_generate_refuses_a_kind_before_the_constants_library() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        kinds: Some(vec![build_vector_kind_document(
            VectorKindDocumentOverrides {
                path_constant_name: Some("G1_add_PATH".to_string()),
                ..Default::default()
            },
        )]),
        constants_library: Some(build_library_document(LibraryDocumentOverrides {
            file_name: Some(String::new()),
            ..Default::default()
        })),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::PathConstantName {
            index: 0,
            error: SolidityConstantNameTryNewErrorReturn::InvalidCharacter {
                index: 3,
                byte: b'a',
            },
        })
    );
    // admit_generate_refuses_a_kind_before_the_constants_library
}

/// Contract: the constants library's file name and name each refused are
///   refused with the file name's refusal.
/// Arrange: `generate` `build_generate_document` with `constants_library`
///   `build_library_document` with `file_name` `""` and `library_name`
///   `"Vector_Records"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::FileName
///   { field: GenerateField::ConstantsLibraryFileName, error:
///   GeneratedFileNameTryNewErrorReturn::Empty })`.
#[test]
fn admit_generate_refuses_the_constants_library_file_name_before_its_library_name() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        constants_library: Some(build_library_document(LibraryDocumentOverrides {
            file_name: Some(String::new()),
            library_name: Some("Vector_Records".to_string()),
        })),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::FileName {
            field: GenerateField::ConstantsLibraryFileName,
            error: GeneratedFileNameTryNewErrorReturn::Empty,
        })
    );
    // admit_generate_refuses_the_constants_library_file_name_before_its_library_name
}

/// Contract: the constants library and the records library each refused are
///   refused with the constants library's refusal.
/// Arrange: `generate` `build_generate_document` with `constants_library`
///   `build_library_document` with `library_name` `"Vector_Records"` and
///   `records_library` `build_library_document` with `file_name` `""`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::LibraryName
///   { field: GenerateField::ConstantsLibraryName, error:
///   SolidityLibraryNameTryNewErrorReturn::InvalidCharacter { index: 6,
///   byte: b'_' } })`.
#[test]
fn admit_generate_refuses_the_constants_library_before_the_records_library() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        constants_library: Some(build_library_document(LibraryDocumentOverrides {
            library_name: Some("Vector_Records".to_string()),
            ..Default::default()
        })),
        records_library: Some(build_library_document(LibraryDocumentOverrides {
            file_name: Some(String::new()),
            ..Default::default()
        })),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::LibraryName {
            field: GenerateField::ConstantsLibraryName,
            error: SolidityLibraryNameTryNewErrorReturn::InvalidCharacter {
                index: 6,
                byte: b'_',
            },
        })
    );
    // admit_generate_refuses_the_constants_library_before_the_records_library
}

/// Contract: the records library's file name and name each refused are
///   refused with the file name's refusal.
/// Arrange: `generate` `build_generate_document` with `records_library`
///   `build_library_document` with `file_name` `""` and `library_name`
///   `"Vector_Records"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::FileName
///   { field: GenerateField::RecordsLibraryFileName, error:
///   GeneratedFileNameTryNewErrorReturn::Empty })`.
#[test]
fn admit_generate_refuses_the_records_library_file_name_before_its_library_name() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        records_library: Some(build_library_document(LibraryDocumentOverrides {
            file_name: Some(String::new()),
            library_name: Some("Vector_Records".to_string()),
        })),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::FileName {
            field: GenerateField::RecordsLibraryFileName,
            error: GeneratedFileNameTryNewErrorReturn::Empty,
        })
    );
    // admit_generate_refuses_the_records_library_file_name_before_its_library_name
}

/// Contract: the records library and a directory segment each refused are
///   refused with the records library's refusal.
/// Arrange: `generate` `build_generate_document` with `records_library`
///   `build_library_document` with `library_name` `"Vector_Records"` and
///   `generated_directory` `vec!["..".to_string()]`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::LibraryName
///   { field: GenerateField::RecordsLibraryName, error:
///   SolidityLibraryNameTryNewErrorReturn::InvalidCharacter { index: 6,
///   byte: b'_' } })`.
#[test]
fn admit_generate_refuses_the_records_library_before_the_directory_segments() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        records_library: Some(build_library_document(LibraryDocumentOverrides {
            library_name: Some("Vector_Records".to_string()),
            ..Default::default()
        })),
        generated_directory: Some(vec!["..".to_string()]),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::LibraryName {
            field: GenerateField::RecordsLibraryName,
            error: SolidityLibraryNameTryNewErrorReturn::InvalidCharacter {
                index: 6,
                byte: b'_',
            },
        })
    );
    // admit_generate_refuses_the_records_library_before_the_directory_segments
}

/// Contract: a directory segment and a curve directory each refused are
///   refused with the segment's refusal.
/// Arrange: `generate` `build_generate_document` with `generated_directory`
///   `vec!["..".to_string()]` and `curve_directories`
///   `build_curve_directories_document` with `bn254` `"bn/254"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::FileName
///   { field: GenerateField::GeneratedDirectorySegment { index: 0 }, error:
///   GeneratedFileNameTryNewErrorReturn::Reserved })`.
#[test]
fn admit_generate_refuses_the_directory_segments_before_the_curve_directories() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        generated_directory: Some(vec!["..".to_string()]),
        curve_directories: Some(build_curve_directories_document(
            CurveDirectoriesDocumentOverrides {
                bn254: Some("bn/254".to_string()),
                ..Default::default()
            },
        )),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::FileName {
            field: GenerateField::GeneratedDirectorySegment { index: 0 },
            error: GeneratedFileNameTryNewErrorReturn::Reserved,
        })
    );
    // admit_generate_refuses_the_directory_segments_before_the_curve_directories
}

/// Contract: both curve directories refused are refused with the bn254
///   directory's refusal.
/// Arrange: `generate` `build_generate_document` with `curve_directories`
///   `build_curve_directories_document` with `bn254` `"bn/254"` and
///   `bls12_381` `"bls/12"`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::FileName
///   { field: GenerateField::Bn254Directory, error:
///   GeneratedFileNameTryNewErrorReturn::InvalidCharacter { index: 2, byte:
///   b'/' } })`.
#[test]
fn admit_generate_refuses_the_bn254_directory_before_the_bls12_381_directory() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        curve_directories: Some(build_curve_directories_document(
            CurveDirectoriesDocumentOverrides {
                bn254: Some("bn/254".to_string()),
                bls12_381: Some("bls/12".to_string()),
            },
        )),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::FileName {
            field: GenerateField::Bn254Directory,
            error: GeneratedFileNameTryNewErrorReturn::InvalidCharacter {
                index: 2,
                byte: b'/',
            },
        })
    );
    // admit_generate_refuses_the_bn254_directory_before_the_bls12_381_directory
}

/// Contract: a curve directory and the license each refused are refused
///   with the curve directory's refusal.
/// Arrange: `generate` `build_generate_document` with `curve_directories`
///   `build_curve_directories_document` with `bls12_381` `"bls/12"` and
///   `license` `""`.
/// Act:    `admit_generate(&deps, params, payload)`.
/// Assert: the result equals `Err(AdmitGenerateErrorReturn::FileName
///   { field: GenerateField::Bls12381Directory, error:
///   GeneratedFileNameTryNewErrorReturn::InvalidCharacter { index: 3, byte:
///   b'/' } })`.
#[test]
fn admit_generate_refuses_the_curve_directories_before_the_license() {
    // Arrange
    let generate = build_generate_document(GenerateDocumentOverrides {
        curve_directories: Some(build_curve_directories_document(
            CurveDirectoriesDocumentOverrides {
                bls12_381: Some("bls/12".to_string()),
                ..Default::default()
            },
        )),
        license: Some(String::new()),
        ..Default::default()
    });
    let deps = build_admit_generate_deps(Default::default());
    let params = build_admit_generate_params(Default::default());
    let payload = build_admit_generate_payload(&generate, Default::default());

    // Act
    let result = admit_generate(&deps, params, payload);

    // Assert
    assert_eq!(
        result,
        Err(AdmitGenerateErrorReturn::FileName {
            field: GenerateField::Bls12381Directory,
            error: GeneratedFileNameTryNewErrorReturn::InvalidCharacter {
                index: 3,
                byte: b'/',
            },
        })
    );
    // admit_generate_refuses_the_curve_directories_before_the_license
}
