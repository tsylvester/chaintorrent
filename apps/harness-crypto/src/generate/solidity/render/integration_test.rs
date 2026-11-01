#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use encoding::CanonicalFieldKind;

use super::interface::SolidityLibraryBody;
use super::mock::{
    RenderParamsOverrides, RenderPayloadOverrides, RenderSuccessReturnOverrides,
    SolidityAbiTypeConstructorParamsOverrides, SolidityCompilerVersionOverrides,
    SolidityConstantNameConstructorParamsOverrides, SolidityLibraryNameConstructorParamsOverrides,
    SolidityMemberNameConstructorParamsOverrides, SolidityRecordDeclarationOverrides,
    SolidityRecordDeclarationsConstructorParamsOverrides, SolidityRecordMemberOverrides,
    SolidityRecordMembersConstructorParamsOverrides, SoliditySourceTextOverrides,
    SolidityStringLiteralConstructorParamsOverrides, SolidityTypeNameConstructorParamsOverrides,
    SpdxLicenseIdentifierConstructorParamsOverrides, build_render_deps, build_render_params,
    build_render_payload, build_render_success_return, build_solidity_abi_type,
    build_solidity_compiler_version, build_solidity_constant_name, build_solidity_library_name,
    build_solidity_member_name, build_solidity_record_declaration,
    build_solidity_record_declarations, build_solidity_record_member,
    build_solidity_record_members, build_solidity_source_text, build_solidity_string_literal,
    build_solidity_type_name, build_spdx_license_identifier,
};
use super::render;

/// Contract: the own entry a reference record library renders exactly.
/// Arrange: `build_render_deps` over defaults; `build_render_params` overriding
///   `license` with `build_spdx_license_identifier` over `GPL-3.0+` and
///   `compiler_version` with `build_solidity_compiler_version` over `7`, `28`,
///   and `315`; `build_render_payload` overriding `library_name` with
///   `build_solidity_library_name` over `ReferenceLib` and `body` with
///   `SolidityLibraryBody::Records` over `build_solidity_record_declarations`
///   whose `declarations` is one `build_solidity_record_declaration` overriding
///   `struct_name` with `build_solidity_type_name` over `Reading`, `members`
///   with `build_solidity_record_members` over `build_solidity_record_member`
///   values named `zeta`, `alpha`, and `mid` of the kinds `Text`,
///   `FixedBytes32`, and `Unsigned64`, carrying `build_solidity_abi_type`
///   values over the names `string`, `bytes32`, and `uint64`,
///   `decode_function_name` with `build_solidity_member_name` over
///   `decodeReading`, `path_constant_name` with `build_solidity_constant_name`
///   over `READING_PATH`, and `path` with `build_solidity_string_literal` over
///   `reading.vec`.
/// Act:    `render` over them.
/// Assert: the return equals `Ok(build_render_success_return(..))` whose
///   `source` is `build_solidity_source_text` over the reference text.
/// Boundary: `render` over the records body, running the real
///   `SolidityAbiType::as_str` and `SolidityRecordMember::decodes_into_memory`.
/// Mocked: nothing, because the chain has no outer-edge collaborator.
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
    let member = |name: &str, kind: CanonicalFieldKind, abi: &str| {
        build_solidity_record_member(SolidityRecordMemberOverrides {
            name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(name.to_string()),
                },
            )),
            kind: Some(kind),
            abi_type: Some(build_solidity_abi_type(
                SolidityAbiTypeConstructorParamsOverrides {
                    name: Some(abi.to_string()),
                },
            )),
        })
    };
    let declaration = build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
        struct_name: Some(build_solidity_type_name(
            SolidityTypeNameConstructorParamsOverrides {
                text: Some("Reading".to_string()),
            },
        )),
        members: Some(build_solidity_record_members(
            SolidityRecordMembersConstructorParamsOverrides {
                members: Some(vec![
                    member("zeta", CanonicalFieldKind::Text, "string"),
                    member("alpha", CanonicalFieldKind::FixedBytes32, "bytes32"),
                    member("mid", CanonicalFieldKind::Unsigned64, "uint64"),
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
    });
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Records(
            build_solidity_record_declarations(
                SolidityRecordDeclarationsConstructorParamsOverrides {
                    declarations: Some(vec![declaration]),
                },
            ),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let expected = build_render_success_return(RenderSuccessReturnOverrides {
        source: Some(build_solidity_source_text(SoliditySourceTextOverrides {
            text: Some(
                "// SPDX-License-Identifier: GPL-3.0+\n\
                 pragma solidity 7.28.315;\n\
                 \n\
                 library ReferenceLib {\n\
                 \x20   struct Reading {\n\
                 \x20       string zeta;\n\
                 \x20       bytes32 alpha;\n\
                 \x20       uint64 mid;\n\
                 \x20   }\n\
                 \n\
                 \x20   string internal constant READING_PATH = \"reading.vec\";\n\
                 \n\
                 \x20   function decodeReading(bytes memory line) internal pure returns (Reading memory) {\n\
                 \x20       (string memory zeta, bytes32 alpha, uint64 mid) = abi.decode(line, (string, bytes32, uint64));\n\
                 \x20       return Reading({zeta: zeta, alpha: alpha, mid: mid});\n\
                 \x20   }\n\
                 }\n"
                    .to_string(),
            ),
        })),
    });
    assert_eq!(result, Ok(expected));
}
// render_produces_the_exact_source_of_a_reference_record_library

/// Contract: the own entry the struct declares one member line per member in
///   order with the ABI type the member carries.
/// Arrange: `build_render_deps` and `build_render_params` over defaults;
///   `build_render_payload` overriding `library_name` with
///   `build_solidity_library_name` over `ReferenceLib` and `body` with
///   `SolidityLibraryBody::Records` over `build_solidity_record_declarations`
///   whose `declarations` is one `build_solidity_record_declaration` overriding
///   `struct_name` with `build_solidity_type_name` over `Reading` and `members`
///   with `build_solidity_record_members` over `build_solidity_record_member`
///   values named `zeta` of `Text`, `alpha` of `FixedBytes32`, `mid` of
///   `Unsigned64`, `omega` of `Bytes`, `beta` of `Unsigned16`, `gamma` of
///   `FixedBytes20`, `delta` of `Unsigned32`, and `epsilon` of `Unsigned256`,
///   carrying `build_solidity_abi_type` values over the names `string`,
///   `bytes32`, `uint64`, `bytes`, `uint16`, `bytes20`, `uint32`, and
///   `uint256`.
/// Act:    `render` over them.
/// Assert: the source text contains `    struct Reading {` and a newline
///   followed by the member lines `        string zeta;`,
///   `        bytes32 alpha;`, `        uint64 mid;`, `        bytes omega;`,
///   `        uint16 beta;`, `        bytes20 gamma;`, `        uint32 delta;`,
///   and `        uint256 epsilon;` in that order, each followed by a newline.
/// Boundary: `render` over the records body, running the real
///   `SolidityAbiType::as_str`.
/// Mocked: nothing, because the chain has no outer-edge collaborator.
#[test]
fn render_declares_the_struct_with_one_member_line_per_member_in_order() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let member = |name: &str, kind: CanonicalFieldKind, abi: &str| {
        build_solidity_record_member(SolidityRecordMemberOverrides {
            name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(name.to_string()),
                },
            )),
            kind: Some(kind),
            abi_type: Some(build_solidity_abi_type(
                SolidityAbiTypeConstructorParamsOverrides {
                    name: Some(abi.to_string()),
                },
            )),
        })
    };
    let declaration = build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
        struct_name: Some(build_solidity_type_name(
            SolidityTypeNameConstructorParamsOverrides {
                text: Some("Reading".to_string()),
            },
        )),
        members: Some(build_solidity_record_members(
            SolidityRecordMembersConstructorParamsOverrides {
                members: Some(vec![
                    member("zeta", CanonicalFieldKind::Text, "string"),
                    member("alpha", CanonicalFieldKind::FixedBytes32, "bytes32"),
                    member("mid", CanonicalFieldKind::Unsigned64, "uint64"),
                    member("omega", CanonicalFieldKind::Bytes, "bytes"),
                    member("beta", CanonicalFieldKind::Unsigned16, "uint16"),
                    member("gamma", CanonicalFieldKind::FixedBytes20, "bytes20"),
                    member("delta", CanonicalFieldKind::Unsigned32, "uint32"),
                    member("epsilon", CanonicalFieldKind::Unsigned256, "uint256"),
                ]),
            },
        )),
        ..Default::default()
    });
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Records(
            build_solidity_record_declarations(
                SolidityRecordDeclarationsConstructorParamsOverrides {
                    declarations: Some(vec![declaration]),
                },
            ),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result else {
        panic!("the payload renders")
    };
    assert!(success.source.as_str().contains(
        "    struct Reading {\n\
         \x20       string zeta;\n\
         \x20       bytes32 alpha;\n\
         \x20       uint64 mid;\n\
         \x20       bytes omega;\n\
         \x20       uint16 beta;\n\
         \x20       bytes20 gamma;\n\
         \x20       uint32 delta;\n\
         \x20       uint256 epsilon;\n"
    ));
}
// render_declares_the_struct_with_one_member_line_per_member_in_order

/// Contract: the own entry the path constant is a string internal constant.
/// Arrange: `build_render_deps` and `build_render_params` over defaults;
///   `build_render_payload` overriding `library_name` with
///   `build_solidity_library_name` over `ReferenceLib` and `body` with
///   `SolidityLibraryBody::Records` over `build_solidity_record_declarations`
///   whose `declarations` is one `build_solidity_record_declaration` overriding
///   `path_constant_name` with `build_solidity_constant_name` over
///   `READING_PATH` and `path` with `build_solidity_string_literal` over
///   `reading.vec`.
/// Act:    `render` over them.
/// Assert: the source text contains the line
///   `    string internal constant READING_PATH = "reading.vec";` and a newline.
/// Boundary: `render` over the records body, running the real
///   `SolidityAbiType::as_str` for the default members.
/// Mocked: nothing, because the chain has no outer-edge collaborator.
#[test]
fn render_writes_the_path_constant_as_a_string_internal_constant() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let declaration = build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
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
    });
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Records(
            build_solidity_record_declarations(
                SolidityRecordDeclarationsConstructorParamsOverrides {
                    declarations: Some(vec![declaration]),
                },
            ),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result else {
        panic!("the payload renders")
    };
    assert!(
        success
            .source
            .as_str()
            .contains("    string internal constant READING_PATH = \"reading.vec\";\n")
    );
}
// render_writes_the_path_constant_as_a_string_internal_constant

/// Contract: the own entry the decode function takes a bytes memory line.
/// Arrange: `build_render_deps` and `build_render_params` over defaults;
///   `build_render_payload` overriding `library_name` with
///   `build_solidity_library_name` over `ReferenceLib` and `body` with
///   `SolidityLibraryBody::Records` over `build_solidity_record_declarations`
///   whose `declarations` is one `build_solidity_record_declaration` overriding
///   `struct_name` with `build_solidity_type_name` over `Reading` and
///   `decode_function_name` with `build_solidity_member_name` over
///   `decodeReading`.
/// Act:    `render` over them.
/// Assert: the source text contains the line
///   `    function decodeReading(bytes memory line) internal pure returns (Reading memory) {`
///   and a newline.
/// Boundary: `render` over the records body, running the real
///   `SolidityAbiType::as_str` for the default members.
/// Mocked: nothing, because the chain has no outer-edge collaborator.
#[test]
fn render_declares_the_decode_function_over_a_bytes_memory_line() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let declaration = build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
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
    });
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Records(
            build_solidity_record_declarations(
                SolidityRecordDeclarationsConstructorParamsOverrides {
                    declarations: Some(vec![declaration]),
                },
            ),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result else {
        panic!("the payload renders")
    };
    assert!(success.source.as_str().contains(
        "    function decodeReading(bytes memory line) internal pure returns (Reading memory) {\n"
    ));
}
// render_declares_the_decode_function_over_a_bytes_memory_line

/// Contract: the own entry only bytes and string locals are marked memory.
/// Arrange: `build_render_deps` and `build_render_params` over defaults;
///   `build_render_payload` overriding `library_name` with
///   `build_solidity_library_name` over `ReferenceLib` and `body` with
///   `SolidityLibraryBody::Records` over `build_solidity_record_declarations`
///   whose `declarations` is one `build_solidity_record_declaration` overriding
///   `members` with `build_solidity_record_members` over
///   `build_solidity_record_member` values named `zeta` of `Text`, `alpha` of
///   `FixedBytes32`, `mid` of `Unsigned64`, `omega` of `Bytes`, `beta` of
///   `Unsigned16`, `gamma` of `FixedBytes20`, `delta` of `Unsigned32`, and
///   `epsilon` of `Unsigned256`, carrying `build_solidity_abi_type` values over
///   the names `string`, `bytes32`, `uint64`, `bytes`, `uint16`, `bytes20`,
///   `uint32`, and `uint256`.
/// Act:    `render` over them.
/// Assert: the source text contains
///   `(string memory zeta, bytes32 alpha, uint64 mid, bytes memory omega, uint16 beta, bytes20 gamma, uint32 delta, uint256 epsilon)`.
/// Boundary: `render` over the records body, running the real
///   `SolidityAbiType::as_str` and `SolidityRecordMember::decodes_into_memory`.
/// Mocked: nothing, because the chain has no outer-edge collaborator.
#[test]
fn render_marks_only_bytes_and_string_locals_as_memory() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let member = |name: &str, kind: CanonicalFieldKind, abi: &str| {
        build_solidity_record_member(SolidityRecordMemberOverrides {
            name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(name.to_string()),
                },
            )),
            kind: Some(kind),
            abi_type: Some(build_solidity_abi_type(
                SolidityAbiTypeConstructorParamsOverrides {
                    name: Some(abi.to_string()),
                },
            )),
        })
    };
    let declaration = build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
        members: Some(build_solidity_record_members(
            SolidityRecordMembersConstructorParamsOverrides {
                members: Some(vec![
                    member("zeta", CanonicalFieldKind::Text, "string"),
                    member("alpha", CanonicalFieldKind::FixedBytes32, "bytes32"),
                    member("mid", CanonicalFieldKind::Unsigned64, "uint64"),
                    member("omega", CanonicalFieldKind::Bytes, "bytes"),
                    member("beta", CanonicalFieldKind::Unsigned16, "uint16"),
                    member("gamma", CanonicalFieldKind::FixedBytes20, "bytes20"),
                    member("delta", CanonicalFieldKind::Unsigned32, "uint32"),
                    member("epsilon", CanonicalFieldKind::Unsigned256, "uint256"),
                ]),
            },
        )),
        ..Default::default()
    });
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Records(
            build_solidity_record_declarations(
                SolidityRecordDeclarationsConstructorParamsOverrides {
                    declarations: Some(vec![declaration]),
                },
            ),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result else {
        panic!("the payload renders")
    };
    assert!(success.source.as_str().contains(
        "(string memory zeta, bytes32 alpha, uint64 mid, bytes memory omega, uint16 beta, bytes20 gamma, uint32 delta, uint256 epsilon)"
    ));
}
// render_marks_only_bytes_and_string_locals_as_memory

/// Contract: the own entry the decode call lists the members' ABI types in
///   member order.
/// Arrange: `build_render_deps` and `build_render_params` over defaults;
///   `build_render_payload` overriding `library_name` with
///   `build_solidity_library_name` over `ReferenceLib` and `body` with
///   `SolidityLibraryBody::Records` over `build_solidity_record_declarations`
///   whose `declarations` is one `build_solidity_record_declaration` overriding
///   `members` with `build_solidity_record_members` over
///   `build_solidity_record_member` values named `zeta` of `Text`, `alpha` of
///   `FixedBytes32`, `mid` of `Unsigned64`, `omega` of `Bytes`, `beta` of
///   `Unsigned16`, `gamma` of `FixedBytes20`, `delta` of `Unsigned32`, and
///   `epsilon` of `Unsigned256`, carrying `build_solidity_abi_type` values over
///   the names `string`, `bytes32`, `uint64`, `bytes`, `uint16`, `bytes20`,
///   `uint32`, and `uint256`.
/// Act:    `render` over them.
/// Assert: the source text contains
///   `) = abi.decode(line, (string, bytes32, uint64, bytes, uint16, bytes20, uint32, uint256));`
///   and a newline.
/// Boundary: `render` over the records body, running the real
///   `SolidityAbiType::as_str`.
/// Mocked: nothing, because the chain has no outer-edge collaborator.
#[test]
fn render_decodes_the_line_over_the_members_abi_types_in_member_order() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let member = |name: &str, kind: CanonicalFieldKind, abi: &str| {
        build_solidity_record_member(SolidityRecordMemberOverrides {
            name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(name.to_string()),
                },
            )),
            kind: Some(kind),
            abi_type: Some(build_solidity_abi_type(
                SolidityAbiTypeConstructorParamsOverrides {
                    name: Some(abi.to_string()),
                },
            )),
        })
    };
    let declaration = build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
        members: Some(build_solidity_record_members(
            SolidityRecordMembersConstructorParamsOverrides {
                members: Some(vec![
                    member("zeta", CanonicalFieldKind::Text, "string"),
                    member("alpha", CanonicalFieldKind::FixedBytes32, "bytes32"),
                    member("mid", CanonicalFieldKind::Unsigned64, "uint64"),
                    member("omega", CanonicalFieldKind::Bytes, "bytes"),
                    member("beta", CanonicalFieldKind::Unsigned16, "uint16"),
                    member("gamma", CanonicalFieldKind::FixedBytes20, "bytes20"),
                    member("delta", CanonicalFieldKind::Unsigned32, "uint32"),
                    member("epsilon", CanonicalFieldKind::Unsigned256, "uint256"),
                ]),
            },
        )),
        ..Default::default()
    });
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Records(
            build_solidity_record_declarations(
                SolidityRecordDeclarationsConstructorParamsOverrides {
                    declarations: Some(vec![declaration]),
                },
            ),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result else {
        panic!("the payload renders")
    };
    assert!(success.source.as_str().contains(
        ") = abi.decode(line, (string, bytes32, uint64, bytes, uint16, bytes20, uint32, uint256));\n"
    ));
}
// render_decodes_the_line_over_the_members_abi_types_in_member_order

/// Contract: the own entry the return builds the struct with named arguments.
/// Arrange: `build_render_deps` and `build_render_params` over defaults;
///   `build_render_payload` overriding `library_name` with
///   `build_solidity_library_name` over `ReferenceLib` and `body` with
///   `SolidityLibraryBody::Records` over `build_solidity_record_declarations`
///   whose `declarations` is one `build_solidity_record_declaration` overriding
///   `struct_name` with `build_solidity_type_name` over `Reading` and `members`
///   with `build_solidity_record_members` over `build_solidity_record_member`
///   values named `zeta` of `Text`, `alpha` of `FixedBytes32`, and `mid` of
///   `Unsigned64`.
/// Act:    `render` over them.
/// Assert: the source text contains the line
///   `        return Reading({zeta: zeta, alpha: alpha, mid: mid});` and a
///   newline.
/// Boundary: `render` over the records body, running the real
///   `SolidityAbiType::as_str`.
/// Mocked: nothing, because the chain has no outer-edge collaborator.
#[test]
fn render_returns_the_struct_built_with_named_arguments() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let member = |name: &str, kind: CanonicalFieldKind| {
        build_solidity_record_member(SolidityRecordMemberOverrides {
            name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(name.to_string()),
                },
            )),
            kind: Some(kind),
            ..Default::default()
        })
    };
    let declaration = build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
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
    });
    let payload = build_render_payload(RenderPayloadOverrides {
        library_name: Some(build_solidity_library_name(
            SolidityLibraryNameConstructorParamsOverrides {
                text: Some("ReferenceLib".to_string()),
            },
        )),
        body: Some(SolidityLibraryBody::Records(
            build_solidity_record_declarations(
                SolidityRecordDeclarationsConstructorParamsOverrides {
                    declarations: Some(vec![declaration]),
                },
            ),
        )),
    });

    // Act
    let result = render(&deps, params, payload);

    // Assert
    let Ok(success) = result else {
        panic!("the payload renders")
    };
    assert!(
        success
            .source
            .as_str()
            .contains("        return Reading({zeta: zeta, alpha: alpha, mid: mid});\n")
    );
}
// render_returns_the_struct_built_with_named_arguments

/// Contract: the own entry the first record block has no leading separator.
/// Arrange: `build_render_deps` and `build_render_params` over defaults;
///   `build_render_payload` overriding `library_name` with
///   `build_solidity_library_name` over `ReferenceLib` and `body` with
///   `SolidityLibraryBody::Records` over `build_solidity_record_declarations`
///   whose `declarations` is one `build_solidity_record_declaration` over
///   defaults.
/// Act:    `render` over them.
/// Assert: the source text contains `library ReferenceLib {` and a newline
///   immediately followed by `    struct `.
/// Boundary: `render` over the records body, running the real
///   `SolidityAbiType::as_str` for the default members.
/// Mocked: nothing, because the chain has no outer-edge collaborator.
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
    let Ok(success) = result else {
        panic!("the payload renders")
    };
    assert!(
        success
            .source
            .as_str()
            .contains("library ReferenceLib {\n    struct ")
    );
}
// render_writes_the_first_record_block_without_a_leading_separator

/// Contract: the own entry record declarations are separated by one empty line.
/// Arrange: `build_render_deps` and `build_render_params` over defaults;
///   `build_render_payload` overriding `library_name` with
///   `build_solidity_library_name` over `ReferenceLib` and `body` with
///   `SolidityLibraryBody::Records` over `build_solidity_record_declarations`
///   whose `declarations` are two `build_solidity_record_declaration` values,
///   the first overriding `struct_name` with `build_solidity_type_name` over
///   `Beta`, `decode_function_name` over `decodeBeta`, `path_constant_name`
///   over `BETA_PATH`, and `path` over `beta.vec`, the second overriding the
///   same slots with `Alpha`, `decodeAlpha`, `ALPHA_PATH`, and `alpha.vec`,
///   each through its name or literal builder.
/// Act:    `render` over them.
/// Assert: the source text contains `    }` and a newline, one empty line, and
///   `    struct Alpha {` and a newline in direct succession.
/// Boundary: `render` over the records body, running the real
///   `SolidityAbiType::as_str` for each declaration's default members.
/// Mocked: nothing, because the chain has no outer-edge collaborator.
#[test]
fn render_separates_record_declarations_by_one_empty_line() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let declaration = |struct_name: &str, decode: &str, path_constant: &str, path: &str| {
        build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
            struct_name: Some(build_solidity_type_name(
                SolidityTypeNameConstructorParamsOverrides {
                    text: Some(struct_name.to_string()),
                },
            )),
            decode_function_name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(decode.to_string()),
                },
            )),
            path_constant_name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(path_constant.to_string()),
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
    let Ok(success) = result else {
        panic!("the payload renders")
    };
    assert!(
        success
            .source
            .as_str()
            .contains("    }\n\n    struct Alpha {\n")
    );
}
// render_separates_record_declarations_by_one_empty_line

/// Contract: the own entry record declarations follow payload order.
/// Arrange: the same two declarations as the separation block, `Beta` then
///   `Alpha`, in `build_render_payload` over `build_render_deps` and
///   `build_render_params` over defaults.
/// Act:    `render` over them.
/// Assert: the offset of `struct Beta` in the source text is lower than the
///   offset of `struct Alpha`.
/// Boundary: `render` over the records body, running the real
///   `SolidityAbiType::as_str` for each declaration's default members.
/// Mocked: nothing, because the chain has no outer-edge collaborator.
#[test]
fn render_writes_record_declarations_in_payload_order() {
    // Arrange
    let deps = build_render_deps(Default::default());
    let params = build_render_params(Default::default());
    let declaration = |struct_name: &str, decode: &str, path_constant: &str, path: &str| {
        build_solidity_record_declaration(SolidityRecordDeclarationOverrides {
            struct_name: Some(build_solidity_type_name(
                SolidityTypeNameConstructorParamsOverrides {
                    text: Some(struct_name.to_string()),
                },
            )),
            decode_function_name: Some(build_solidity_member_name(
                SolidityMemberNameConstructorParamsOverrides {
                    text: Some(decode.to_string()),
                },
            )),
            path_constant_name: Some(build_solidity_constant_name(
                SolidityConstantNameConstructorParamsOverrides {
                    text: Some(path_constant.to_string()),
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
    let Ok(success) = result else {
        panic!("the payload renders")
    };
    let text = success.source.as_str();
    let Some(beta_offset) = text.find("struct Beta") else {
        panic!("the first block is written")
    };
    let Some(alpha_offset) = text.find("struct Alpha") else {
        panic!("the second block is written")
    };
    assert!(beta_offset < alpha_offset);
}
// render_writes_record_declarations_in_payload_order
