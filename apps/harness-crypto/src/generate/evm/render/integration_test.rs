use super::provides::*;
use encoding::CanonicalFieldKind;

/// Contract: the own entry, a reference record library renders exactly: the
///   source is the header, the declaration's block of struct, path constant,
///   and decode function in that order, and the closing brace.
/// Arrange: `build_render_deps(Default::default())`; `build_render_params`
///   overriding `license` with `build_spdx_license_identifier` over
///   `GPL-3.0+` and `compiler_version` with `build_solidity_compiler_version`
///   over `7`, `28`, and `315`; `build_render_payload` overriding
///   `library_name` with `build_solidity_library_name` over `ReferenceLib`
///   and `body` with `SolidityLibraryBody::Records` over
///   `build_solidity_record_declarations` whose `declarations` is one
///   `build_solidity_record_declaration` overriding `struct_name` with
///   `build_solidity_type_name` over `Reading`, `members` with
///   `build_solidity_record_members` over `build_solidity_record_member`
///   values named `zeta`, `alpha`, and `mid` of the kinds `Text`,
///   `FixedBytes32`, and `Unsigned64`, `decode_function_name` with
///   `build_solidity_member_name` over `decodeReading`, `path_constant_name`
///   with `build_solidity_constant_name` over `READING_PATH`, and `path`
///   with `build_solidity_string_literal` over `reading.vec`; each slot of
///   the declaration carries a different value, so a swap of struct name,
///   decode function name, path constant name, and path fails.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  `result.ok()` equals `Some` of `build_render_success_return`
///   whose `source` is `build_solidity_source_text` over the reference text,
///   a literal the block assembles from the spec's header, block, and
///   closing lines, by `assert_eq!`.
/// Boundary: `render` over the records body, running the real
///   `SolidityAbiType::try_new`, `as_str`, and `decodes_into_memory`.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn render_produces_the_exact_source_of_a_reference_record_library() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(RenderParamsOverrides {
        license: Some(build_spdx_license_identifier(
            SpdxLicenseIdentifierConstructorParamsOverrides {
                text: Some("GPL-3.0+".to_string()),
            },
        )),
        compiler_version: Some(build_solidity_compiler_version(
            SolidityCompilerVersionOverrides {
                major: Some(7),
                minor: Some(28),
                patch: Some(315),
            },
        )),
    });
    let member = |text: &str, kind: CanonicalFieldKind| {
        build_solidity_record_member(SolidityRecordMemberOverrides {
            name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(text.to_string()),
                },
            )),
            kind: Some(kind),
        })
    };
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Records(
            build_solidity_record_declarations(
                SolidityRecordDeclarationsConstructorParamsOverrides {
                    declarations: Some(vec![build_solidity_record_declaration(
                        SolidityRecordDeclarationOverrides {
                            struct_name: Some(build_solidity_type_name(
                                SolidityTypeNameConstructorParamsOverrides {
                                    text: Some("Reading".to_string()),
                                },
                            )),
                            members: Some(build_solidity_record_members(
                                SolidityRecordMembersConstructorParamsOverrides {
                                    members: Some(vec![
                                        member("zeta", CanonicalFieldKind::Text),
                                        member("alpha", CanonicalFieldKind::FixedBytes32),
                                        member("mid", CanonicalFieldKind::Unsigned64),
                                    ]),
                                },
                            )),
                            decode_function_name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("decodeReading".to_string()),
                                },
                            )),
                            path_constant_name: Some(build_solidity_constant_name(
                                SolidityConstantNameConstructorParamsOverrides {
                                    text: Some("READING_PATH".to_string()),
                                },
                            )),
                            path: Some(build_solidity_string_literal(
                                SolidityStringLiteralConstructorParamsOverrides {
                                    text: Some("reading.vec".to_string()),
                                },
                            )),
                        },
                    )]),
                },
            ),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    assert_eq!(
        result.ok(),
        Some(build_render_success_return(RenderSuccessReturnOverrides {
            source: Some(build_solidity_source_text(SoliditySourceTextOverrides {
                text: Some(
                    concat!(
                        "// SPDX-License-Identifier: GPL-3.0+\n",
                        "pragma solidity 7.28.315;\n",
                        "\n",
                        "library ReferenceLib {\n",
                        "    struct Reading {\n",
                        "        string zeta;\n",
                        "        bytes32 alpha;\n",
                        "        uint64 mid;\n",
                        "    }\n",
                        "\n",
                        "    string internal constant READING_PATH = \"reading.vec\";\n",
                        "\n",
                        "    function decodeReading(bytes memory line) internal pure returns (Reading memory) {\n",
                        "        (string memory zeta, bytes32 alpha, uint64 mid) = abi.decode(line, (string, bytes32, uint64));\n",
                        "        return Reading({zeta: zeta, alpha: alpha, mid: mid});\n",
                        "    }\n",
                        "}\n"
                    )
                    .to_string()
                ),
            })),
        }))
    );
}

/// Contract: the own entry, the struct declares one member line per member
///   in order with the table's ABI type: eight spaces, the ABI type
///   `SolidityAbiType::as_str` names for the member's kind, a space, the
///   member name, and `;`.
/// Arrange: `build_render_deps(Default::default())`;
///   `build_render_params(Default::default())`; `build_render_payload`
///   overriding `library_name` with `build_solidity_library_name` over
///   `ReferenceLib` and `body` with `SolidityLibraryBody::Records` over
///   `build_solidity_record_declarations` whose `declarations` is one
///   `build_solidity_record_declaration` overriding `struct_name` with
///   `build_solidity_type_name` over `Reading` and `members` with
///   `build_solidity_record_members` over `build_solidity_record_member`
///   values named `zeta` of `Text`, `alpha` of `FixedBytes32`, `mid` of
///   `Unsigned64`, `omega` of `Bytes`, `beta` of `Unsigned16`, `gamma` of
///   `FixedBytes20`, `delta` of `Unsigned32`, and `epsilon` of
///   `Unsigned256`; the members are unsorted and cover every kind, so a
///   kind mapped to another kind's name or a reordered member fails.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  `source.as_str()` contains `    struct Reading {` and a newline
///   followed by the member lines `        string zeta;`,
///   `        bytes32 alpha;`, `        uint64 mid;`, `        bytes omega;`,
///   `        uint16 beta;`, `        bytes20 gamma;`,
///   `        uint32 delta;`, and `        uint256 epsilon;` in that order,
///   each followed by a newline.
/// Boundary: `render` over the records body, running the real
///   `SolidityAbiType::try_new` and `as_str`.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn render_declares_the_struct_with_one_member_line_per_member_in_order() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let member = |text: &str, kind: CanonicalFieldKind| {
        build_solidity_record_member(SolidityRecordMemberOverrides {
            name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(text.to_string()),
                },
            )),
            kind: Some(kind),
        })
    };
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Records(
            build_solidity_record_declarations(
                SolidityRecordDeclarationsConstructorParamsOverrides {
                    declarations: Some(vec![build_solidity_record_declaration(
                        SolidityRecordDeclarationOverrides {
                            struct_name: Some(build_solidity_type_name(
                                SolidityTypeNameConstructorParamsOverrides {
                                    text: Some("Reading".to_string()),
                                },
                            )),
                            members: Some(build_solidity_record_members(
                                SolidityRecordMembersConstructorParamsOverrides {
                                    members: Some(vec![
                                        member("zeta", CanonicalFieldKind::Text),
                                        member("alpha", CanonicalFieldKind::FixedBytes32),
                                        member("mid", CanonicalFieldKind::Unsigned64),
                                        member("omega", CanonicalFieldKind::Bytes),
                                        member("beta", CanonicalFieldKind::Unsigned16),
                                        member("gamma", CanonicalFieldKind::FixedBytes20),
                                        member("delta", CanonicalFieldKind::Unsigned32),
                                        member("epsilon", CanonicalFieldKind::Unsigned256),
                                    ]),
                                },
                            )),
                            ..Default::default()
                        },
                    )]),
                },
            ),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(success.source.as_str().contains(concat!(
        "    struct Reading {\n",
        "        string zeta;\n",
        "        bytes32 alpha;\n",
        "        uint64 mid;\n",
        "        bytes omega;\n",
        "        uint16 beta;\n",
        "        bytes20 gamma;\n",
        "        uint32 delta;\n",
        "        uint256 epsilon;\n",
    )));
}

/// Contract: the own entry, the path constant is a string internal
///   constant: `    string internal constant `, the path constant name,
///   ` = "`, the path's text, and `";` with a newline, the name and the
///   path being different texts so a swap fails.
/// Arrange: `build_render_deps(Default::default())`;
///   `build_render_params(Default::default())`; `build_render_payload`
///   overriding `library_name` with `build_solidity_library_name` over
///   `ReferenceLib` and `body` with `SolidityLibraryBody::Records` over
///   `build_solidity_record_declarations` whose `declarations` is one
///   `build_solidity_record_declaration` overriding `path_constant_name`
///   with `build_solidity_constant_name` over `READING_PATH` and `path`
///   with `build_solidity_string_literal` over `reading.vec`.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  `source.as_str()` contains the line `    string internal
///   constant READING_PATH = "reading.vec";` and a newline.
/// Boundary: `render` over the records body, running the real
///   `SolidityAbiType::try_new` and `as_str` for the default members.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn render_writes_the_path_constant_as_a_string_internal_constant() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Records(
            build_solidity_record_declarations(
                SolidityRecordDeclarationsConstructorParamsOverrides {
                    declarations: Some(vec![build_solidity_record_declaration(
                        SolidityRecordDeclarationOverrides {
                            path_constant_name: Some(build_solidity_constant_name(
                                SolidityConstantNameConstructorParamsOverrides {
                                    text: Some("READING_PATH".to_string()),
                                },
                            )),
                            path: Some(build_solidity_string_literal(
                                SolidityStringLiteralConstructorParamsOverrides {
                                    text: Some("reading.vec".to_string()),
                                },
                            )),
                            ..Default::default()
                        },
                    )]),
                },
            ),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(
        success
            .source
            .as_str()
            .contains("    string internal constant READING_PATH = \"reading.vec\";\n")
    );
}

/// Contract: the own entry, the decode function takes a bytes memory line:
///   `    function `, the decode function name, `(bytes memory line)
///   internal pure returns (`, the struct name, and ` memory) {` with a
///   newline, the two names being different texts so a swap fails.
/// Arrange: `build_render_deps(Default::default())`;
///   `build_render_params(Default::default())`; `build_render_payload`
///   overriding `library_name` with `build_solidity_library_name` over
///   `ReferenceLib` and `body` with `SolidityLibraryBody::Records` over
///   `build_solidity_record_declarations` whose `declarations` is one
///   `build_solidity_record_declaration` overriding `struct_name` with
///   `build_solidity_type_name` over `Reading` and `decode_function_name`
///   with `build_solidity_member_name` over `decodeReading`.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  `source.as_str()` contains the line `    function
///   decodeReading(bytes memory line) internal pure returns (Reading
///   memory) {` and a newline.
/// Boundary: `render` over the records body, running the real
///   `SolidityAbiType::try_new` and `as_str` for the default members.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn render_declares_the_decode_function_over_a_bytes_memory_line() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Records(
            build_solidity_record_declarations(
                SolidityRecordDeclarationsConstructorParamsOverrides {
                    declarations: Some(vec![build_solidity_record_declaration(
                        SolidityRecordDeclarationOverrides {
                            struct_name: Some(build_solidity_type_name(
                                SolidityTypeNameConstructorParamsOverrides {
                                    text: Some("Reading".to_string()),
                                },
                            )),
                            decode_function_name: Some(build_solidity_member_name(
                                SolidityMemberNameConstructorParamsOverrides {
                                    text: Some("decodeReading".to_string()),
                                },
                            )),
                            ..Default::default()
                        },
                    )]),
                },
            ),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(success.source.as_str().contains(
        "    function decodeReading(bytes memory line) internal pure returns (Reading memory) {\n"
    ));
}

/// Contract: the own entry, only bytes and string locals are marked memory:
///   the destructuring line has one local per member in order, the ABI
///   type, ` memory` where `SolidityAbiType::decodes_into_memory` returns
///   `true`, a space, and the member name; every kind is present, so a kind
///   marked memory that is not `Text` or `Bytes`, or one that is and is not
///   marked, fails.
/// Arrange: `build_render_deps(Default::default())`;
///   `build_render_params(Default::default())`; `build_render_payload`
///   overriding `library_name` with `build_solidity_library_name` over
///   `ReferenceLib` and `body` with `SolidityLibraryBody::Records` over
///   `build_solidity_record_declarations` whose `declarations` is one
///   `build_solidity_record_declaration` overriding `members` with
///   `build_solidity_record_members` over `build_solidity_record_member`
///   values named `zeta` of `Text`, `alpha` of `FixedBytes32`, `mid` of
///   `Unsigned64`, `omega` of `Bytes`, `beta` of `Unsigned16`, `gamma` of
///   `FixedBytes20`, `delta` of `Unsigned32`, and `epsilon` of
///   `Unsigned256`.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  `source.as_str()` contains `(string memory zeta, bytes32 alpha,
///   uint64 mid, bytes memory omega, uint16 beta, bytes20 gamma, uint32
///   delta, uint256 epsilon)`.
/// Boundary: `render` over the records body, running the real
///   `SolidityAbiType::try_new`, `as_str`, and `decodes_into_memory`.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn render_marks_only_bytes_and_string_locals_as_memory() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let member = |text: &str, kind: CanonicalFieldKind| {
        build_solidity_record_member(SolidityRecordMemberOverrides {
            name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(text.to_string()),
                },
            )),
            kind: Some(kind),
        })
    };
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Records(
            build_solidity_record_declarations(
                SolidityRecordDeclarationsConstructorParamsOverrides {
                    declarations: Some(vec![build_solidity_record_declaration(
                        SolidityRecordDeclarationOverrides {
                            members: Some(build_solidity_record_members(
                                SolidityRecordMembersConstructorParamsOverrides {
                                    members: Some(vec![
                                        member("zeta", CanonicalFieldKind::Text),
                                        member("alpha", CanonicalFieldKind::FixedBytes32),
                                        member("mid", CanonicalFieldKind::Unsigned64),
                                        member("omega", CanonicalFieldKind::Bytes),
                                        member("beta", CanonicalFieldKind::Unsigned16),
                                        member("gamma", CanonicalFieldKind::FixedBytes20),
                                        member("delta", CanonicalFieldKind::Unsigned32),
                                        member("epsilon", CanonicalFieldKind::Unsigned256),
                                    ]),
                                },
                            )),
                            ..Default::default()
                        },
                    )]),
                },
            ),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(success.source.as_str().contains(
        "(string memory zeta, bytes32 alpha, uint64 mid, bytes memory omega, uint16 beta, bytes20 gamma, uint32 delta, uint256 epsilon)"
    ));
}

/// Contract: the own entry, the decode call lists the members' ABI types in
///   member order: the destructuring line ends in `) = abi.decode(line, (`
///   the members' ABI types joined by `, ` and `));`, the list and the
///   locals naming the same type for each member.
/// Arrange: `build_render_deps(Default::default())`;
///   `build_render_params(Default::default())`; `build_render_payload`
///   overriding `library_name` with `build_solidity_library_name` over
///   `ReferenceLib` and `body` with `SolidityLibraryBody::Records` over
///   `build_solidity_record_declarations` whose `declarations` is one
///   `build_solidity_record_declaration` overriding `members` with
///   `build_solidity_record_members` over `build_solidity_record_member`
///   values named `zeta` of `Text`, `alpha` of `FixedBytes32`, `mid` of
///   `Unsigned64`, `omega` of `Bytes`, `beta` of `Unsigned16`, `gamma` of
///   `FixedBytes20`, `delta` of `Unsigned32`, and `epsilon` of
///   `Unsigned256`, unsorted members of every kind.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  `source.as_str()` contains `) = abi.decode(line, (string,
///   bytes32, uint64, bytes, uint16, bytes20, uint32, uint256));` and a
///   newline.
/// Boundary: `render` over the records body, running the real
///   `SolidityAbiType::try_new` and `as_str`.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn render_decodes_the_line_over_the_members_abi_types_in_member_order() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let member = |text: &str, kind: CanonicalFieldKind| {
        build_solidity_record_member(SolidityRecordMemberOverrides {
            name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(text.to_string()),
                },
            )),
            kind: Some(kind),
        })
    };
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Records(
            build_solidity_record_declarations(
                SolidityRecordDeclarationsConstructorParamsOverrides {
                    declarations: Some(vec![build_solidity_record_declaration(
                        SolidityRecordDeclarationOverrides {
                            members: Some(build_solidity_record_members(
                                SolidityRecordMembersConstructorParamsOverrides {
                                    members: Some(vec![
                                        member("zeta", CanonicalFieldKind::Text),
                                        member("alpha", CanonicalFieldKind::FixedBytes32),
                                        member("mid", CanonicalFieldKind::Unsigned64),
                                        member("omega", CanonicalFieldKind::Bytes),
                                        member("beta", CanonicalFieldKind::Unsigned16),
                                        member("gamma", CanonicalFieldKind::FixedBytes20),
                                        member("delta", CanonicalFieldKind::Unsigned32),
                                        member("epsilon", CanonicalFieldKind::Unsigned256),
                                    ]),
                                },
                            )),
                            ..Default::default()
                        },
                    )]),
                },
            ),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(success.source.as_str().contains(
        ") = abi.decode(line, (string, bytes32, uint64, bytes, uint16, bytes20, uint32, uint256));\n"
    ));
}

/// Contract: the own entry, the return builds the struct with named
///   arguments: the block ends with eight spaces, `return `, the struct
///   name, `({`, each member as its name, `: `, and its name joined by
///   `, `, and `});` with a newline, the arguments following the members
///   whatever their kinds.
/// Arrange: `build_render_deps(Default::default())`;
///   `build_render_params(Default::default())`; `build_render_payload`
///   overriding `library_name` with `build_solidity_library_name` over
///   `ReferenceLib` and `body` with `SolidityLibraryBody::Records` over
///   `build_solidity_record_declarations` whose `declarations` is one
///   `build_solidity_record_declaration` overriding `struct_name` with
///   `build_solidity_type_name` over `Reading` and `members` with
///   `build_solidity_record_members` over `build_solidity_record_member`
///   values named `zeta` of `Text`, `alpha` of `FixedBytes32`, and `mid` of
///   `Unsigned64`, unsorted member names.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  `source.as_str()` contains the line `        return
///   Reading({zeta: zeta, alpha: alpha, mid: mid});` and a newline.
/// Boundary: `render` over the records body, running the real
///   `SolidityAbiType::try_new` and `as_str`.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn render_returns_the_struct_built_with_named_arguments() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let member = |text: &str, kind: CanonicalFieldKind| {
        build_solidity_record_member(SolidityRecordMemberOverrides {
            name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(text.to_string()),
                },
            )),
            kind: Some(kind),
        })
    };
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Records(
            build_solidity_record_declarations(
                SolidityRecordDeclarationsConstructorParamsOverrides {
                    declarations: Some(vec![build_solidity_record_declaration(
                        SolidityRecordDeclarationOverrides {
                            struct_name: Some(build_solidity_type_name(
                                SolidityTypeNameConstructorParamsOverrides {
                                    text: Some("Reading".to_string()),
                                },
                            )),
                            members: Some(build_solidity_record_members(
                                SolidityRecordMembersConstructorParamsOverrides {
                                    members: Some(vec![
                                        member("zeta", CanonicalFieldKind::Text),
                                        member("alpha", CanonicalFieldKind::FixedBytes32),
                                        member("mid", CanonicalFieldKind::Unsigned64),
                                    ]),
                                },
                            )),
                            ..Default::default()
                        },
                    )]),
                },
            ),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(
        success
            .source
            .as_str()
            .contains("        return Reading({zeta: zeta, alpha: alpha, mid: mid});\n")
    );
}

/// Contract: the own entry, the first record block has no leading
///   separator: the separator is written between blocks only.
/// Arrange: `build_render_deps(Default::default())`;
///   `build_render_params(Default::default())`; `build_render_payload`
///   overriding `library_name` with `build_solidity_library_name` over
///   `ReferenceLib` and `body` with `SolidityLibraryBody::Records` over
///   `build_solidity_record_declarations` whose `declarations` is one
///   `build_solidity_record_declaration(Default::default())`.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  `source.as_str()` contains `library ReferenceLib {` and a
///   newline immediately followed by `    struct `.
/// Boundary: `render` over the records body, running the real
///   `SolidityAbiType::try_new` and `as_str` for the default members.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn render_writes_the_first_record_block_without_a_leading_separator() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Records(
            build_solidity_record_declarations(
                SolidityRecordDeclarationsConstructorParamsOverrides {
                    declarations: Some(vec![build_solidity_record_declaration(Default::default())]),
                },
            ),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(
        success
            .source
            .as_str()
            .contains("library ReferenceLib {\n    struct ")
    );
}

/// Contract: the own entry, record declarations are separated by one empty
///   line: the second block is written exactly as the first.
/// Arrange: `build_render_deps(Default::default())`;
///   `build_render_params(Default::default())`; `build_render_payload`
///   overriding `library_name` with `build_solidity_library_name` over
///   `ReferenceLib` and `body` with `SolidityLibraryBody::Records` over
///   `build_solidity_record_declarations` whose `declarations` are two
///   `build_solidity_record_declaration` values, the first overriding
///   `struct_name` with `build_solidity_type_name` over `Beta`,
///   `decode_function_name` over `decodeBeta`, `path_constant_name` over
///   `BETA_PATH`, and `path` over `beta.vec`, the second overriding the
///   same slots with `Alpha`, `decodeAlpha`, `ALPHA_PATH`, and `alpha.vec`,
///   each through its name or literal builder.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  `source.as_str()` contains `    }` and a newline, one empty
///   line, and `    struct Alpha {` and a newline in direct succession.
/// Boundary: `render` over the records body, running the real
///   `SolidityAbiType::try_new` and `as_str` for each declaration's default
///   members.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn render_separates_record_declarations_by_one_empty_line() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let declaration =
        |struct_name: &str, decode_function_name: &str, path_constant_name: &str, path: &str| {
            build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
                struct_name: Some(build_solidity_type_name(
                    SolidityTypeNameConstructorParamsOverrides {
                        text: Some(struct_name.to_string()),
                    },
                )),
                decode_function_name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some(decode_function_name.to_string()),
                    },
                )),
                path_constant_name: Some(build_solidity_constant_name(
                    SolidityConstantNameConstructorParamsOverrides {
                        text: Some(path_constant_name.to_string()),
                    },
                )),
                path: Some(build_solidity_string_literal(
                    SolidityStringLiteralConstructorParamsOverrides {
                        text: Some(path.to_string()),
                    },
                )),
                ..Default::default()
            })
        };
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Records(
            build_solidity_record_declarations(
                SolidityRecordDeclarationsConstructorParamsOverrides {
                    declarations: Some(vec![
                        declaration("Beta", "decodeBeta", "BETA_PATH", "beta.vec"),
                        declaration("Alpha", "decodeAlpha", "ALPHA_PATH", "alpha.vec"),
                    ]),
                },
            ),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    assert!(
        success
            .source
            .as_str()
            .contains("    }\n\n    struct Alpha {\n")
    );
}

/// Contract: the own entry, record declarations follow payload order: the
///   first declaration's block precedes the second's, the order being the
///   payload's.
/// Arrange: `build_render_deps(Default::default())` and
///   `build_render_params(Default::default())`; `build_render_payload`
///   overriding `library_name` with `build_solidity_library_name` over
///   `ReferenceLib` and `body` with `SolidityLibraryBody::Records` over the
///   same two declarations as the separation block, `Beta` then `Alpha`.
/// Act:     `render(&deps, params, payload)`.
/// Assert:  the offset of `struct Beta` in `source.as_str()` is lower than
///   the offset of `struct Alpha`.
/// Boundary: `render` over the records body, running the real
///   `SolidityAbiType::try_new` and `as_str` for each declaration's default
///   members.
/// Mocked:  nothing, because the chain has no outer-edge collaborator.
#[test]
fn render_writes_record_declarations_in_payload_order() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let declaration =
        |struct_name: &str, decode_function_name: &str, path_constant_name: &str, path: &str| {
            build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
                struct_name: Some(build_solidity_type_name(
                    SolidityTypeNameConstructorParamsOverrides {
                        text: Some(struct_name.to_string()),
                    },
                )),
                decode_function_name: Some(build_solidity_member_name(
                    SolidityMemberNameConstructorParamsOverrides {
                        text: Some(decode_function_name.to_string()),
                    },
                )),
                path_constant_name: Some(build_solidity_constant_name(
                    SolidityConstantNameConstructorParamsOverrides {
                        text: Some(path_constant_name.to_string()),
                    },
                )),
                path: Some(build_solidity_string_literal(
                    SolidityStringLiteralConstructorParamsOverrides {
                        text: Some(path.to_string()),
                    },
                )),
                ..Default::default()
            })
        };
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Records(
            build_solidity_record_declarations(
                SolidityRecordDeclarationsConstructorParamsOverrides {
                    declarations: Some(vec![
                        declaration("Beta", "decodeBeta", "BETA_PATH", "beta.vec"),
                        declaration("Alpha", "decodeAlpha", "ALPHA_PATH", "alpha.vec"),
                    ]),
                },
            ),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result;
    let beta_at = success.source.as_str().find("struct Beta");
    let alpha_at = success.source.as_str().find("struct Alpha");
    assert!(beta_at.is_some() && beta_at < alpha_at);
}
