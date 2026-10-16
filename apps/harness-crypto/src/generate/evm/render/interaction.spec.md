# render interactions

`render` assembles the source text of one Solidity library from a license identifier, a compiler version, a library name, and either a list of named constant entries or a list of vector record declarations. It reads no dependency, draws no randomness, reads no clock, and touches no filesystem; every refusal is a constructor's. The validated name, literal, value, entry, member, and declaration types admit only text and lists the rendering can emit, so the function itself has no failure branch.

## `SolidityLibraryName::try_new`

`SolidityLibraryName::try_new(params: SolidityLibraryNameConstructorParams) -> SolidityLibraryNameTryNewReturn`

Decision: over `params.text.as_bytes()`.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| empty | no byte | the text's length is zero | none | `Err(SolidityLibraryNameTryNewErrorReturn::Empty)` |
| leading byte | the byte at index `0` is outside `b'A'..=b'Z'` | the first byte fails the leading set | none | `Err(SolidityLibraryNameTryNewErrorReturn::LeadingCharacter { byte })` |
| later byte | the lowest index `i` from `1` whose byte is outside `b'A'..=b'Z'`, `b'a'..=b'z'`, and `b'0'..=b'9'` | the lowest failing later byte | none | `Err(SolidityLibraryNameTryNewErrorReturn::InvalidCharacter { index: i, byte })` |
| admitted | no branch above | every byte is in its set | none | `Ok(SolidityLibraryName { text })`, the text moved from the params |

## `SolidityConstantName::try_new`

`SolidityConstantName::try_new(params: SolidityConstantNameConstructorParams) -> SolidityConstantNameTryNewReturn`

Decision: the same branches in the same order as `SolidityLibraryName::try_new` with `SolidityConstantNameTryNewErrorReturn`'s variants, the later-byte set being `b'A'..=b'Z'`, `b'0'..=b'9'`, and `b'_'`; admitted, outcome `Ok(SolidityConstantName { text })`.

## `SpdxLicenseIdentifier::try_new`

`SpdxLicenseIdentifier::try_new(params: SpdxLicenseIdentifierConstructorParams) -> SpdxLicenseIdentifierTryNewReturn`

Decision: the identifier part is the text without its last byte when that byte is `b'+'`, and the whole text otherwise; branches are decided over the identifier part, indexes into the whole text.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| empty | the identifier part has no byte | the text is empty or a lone `+` | none | `Err(SpdxLicenseIdentifierTryNewErrorReturn::Empty)` |
| invalid | the lowest index `i` of the identifier part whose byte is outside `b'A'..=b'Z'`, `b'a'..=b'z'`, `b'0'..=b'9'`, `b'.'`, and `b'-'` | the lowest failing byte of the identifier part | none | `Err(SpdxLicenseIdentifierTryNewErrorReturn::InvalidCharacter { index: i, byte })` |
| admitted | no branch above | the identifier part is non-empty and in its set | none | `Ok(SpdxLicenseIdentifier { text })`, the whole text including a final `+` |

## `SolidityStringLiteral::try_new`

`SolidityStringLiteral::try_new(params: SolidityStringLiteralConstructorParams) -> SolidityStringLiteralTryNewReturn`

Decision: over `params.text.as_bytes()`.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| invalid | the lowest index `i` whose byte is outside `0x20..=0x7E` or equals `b'"'` or `b'\\'` | the lowest failing byte | none | `Err(SolidityStringLiteralTryNewErrorReturn::InvalidCharacter { index: i, byte })` |
| admitted | no branch above | every byte is in its set; the empty string is admitted | none | `Ok(SolidityStringLiteral { text })`, the text moved from the params |

## `SolidityUint256::try_new`

`SolidityUint256::try_new(params: SolidityUint256ConstructorParams) -> SolidityUint256TryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| admitted | always | one branch | none | `Ok(SolidityUint256 { big_endian })`, the array moved from the params |

## `SolidityLibraryEntries::try_new`

`SolidityLibraryEntries::try_new(params: SolidityLibraryEntriesConstructorParams) -> SolidityLibraryEntriesTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| empty | `params.entries` is empty | the vector's length is zero | none | `Err(SolidityLibraryEntriesTryNewErrorReturn::Empty)` |
| duplicate | the lowest index `j` whose entry's `name` equals the `name` of an entry at a lower index | `i` is the lowest such lower index | none | `Err(SolidityLibraryEntriesTryNewErrorReturn::DuplicateName { first_index: i, duplicate_index: j })` |
| admitted | no branch above | every name is unique | none | `Ok(SolidityLibraryEntries { entries })`, the vector moved from the params in its order |

## `SolidityTypeName::try_new`

`SolidityTypeName::try_new(params: SolidityTypeNameConstructorParams) -> SolidityTypeNameTryNewReturn`

Decision: the same branches in the same order as `SolidityLibraryName::try_new` with `SolidityTypeNameTryNewErrorReturn`'s variants, the leading set `b'A'..=b'Z'` and the later-byte set `b'A'..=b'Z'`, `b'a'..=b'z'`, and `b'0'..=b'9'`; admitted, outcome `Ok(SolidityTypeName { text })`.

## `SolidityMemberName::try_new`

`SolidityMemberName::try_new(params: SolidityMemberNameConstructorParams) -> SolidityMemberNameTryNewReturn`

Decision: the same branches in the same order as `SolidityLibraryName::try_new` with `SolidityMemberNameTryNewErrorReturn`'s variants, the leading set `b'a'..=b'z'` and the later-byte set `b'A'..=b'Z'`, `b'a'..=b'z'`, and `b'0'..=b'9'`; admitted, outcome `Ok(SolidityMemberName { text })`.

## `SolidityAbiType::try_new`

`SolidityAbiType::try_new(params: SolidityAbiTypeConstructorParams) -> SolidityAbiTypeTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| admitted | always | one branch | none | `Ok(SolidityAbiType { kind })`, the kind moved from the params |

## `SolidityAbiType::as_str`

`SolidityAbiType::as_str(&self) -> &'static str`

Decision: an exhaustive `match` on `self.kind`, one arm per `CanonicalFieldKind` variant, so a variant with no arm fails to compile: `FixedBytes32` to `bytes32`, `Unsigned16` to `uint16`, `Unsigned32` to `uint32`, `Unsigned64` to `uint64`, `Text` to `string`, `Bytes` to `bytes`, `FixedBytes20` to `bytes20`, and `Unsigned256` to `uint256`.

## `SolidityAbiType::decodes_into_memory`

`SolidityAbiType::decodes_into_memory(&self) -> bool`

Decision: an exhaustive `match` on `self.kind`, `Text` and `Bytes` to `true` and every other kind to `false`.

## `SolidityRecordMembers::try_new`

`SolidityRecordMembers::try_new(params: SolidityRecordMembersConstructorParams) -> SolidityRecordMembersTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| too few | `params.members` holds fewer than two members | the vector's length is under two | none | `Err(SolidityRecordMembersTryNewErrorReturn::TooFew { actual })`, `actual` the length |
| reserved | the lowest index `k` whose member's name text equals `line` | the decode function's parameter name is taken | none | `Err(SolidityRecordMembersTryNewErrorReturn::ReservedName { index: k })` |
| duplicate | the lowest index `j` whose member's `name` equals the `name` of a member at a lower index | `i` is the lowest such lower index | none | `Err(SolidityRecordMembersTryNewErrorReturn::DuplicateName { first_index: i, duplicate_index: j })` |
| admitted | no branch above | at least two members, none named `line`, every name unique | none | `Ok(SolidityRecordMembers { members })`, the vector moved from the params in its order |

## `SolidityRecordDeclarations::try_new`

`SolidityRecordDeclarations::try_new(params: SolidityRecordDeclarationsConstructorParams) -> SolidityRecordDeclarationsTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| empty | `params.declarations` is empty | the vector's length is zero | none | `Err(SolidityRecordDeclarationsTryNewErrorReturn::Empty)` |
| duplicate struct name | the lowest index `j` whose `struct_name` equals the `struct_name` of a declaration at a lower index | `i` is the lowest such lower index | none | `Err(SolidityRecordDeclarationsTryNewErrorReturn::DuplicateStructName { first_index: i, duplicate_index: j })` |
| duplicate decode function name | the lowest index `j` whose `decode_function_name` equals the `decode_function_name` of a declaration at a lower index | `i` is the lowest such lower index | none | `Err(SolidityRecordDeclarationsTryNewErrorReturn::DuplicateDecodeFunctionName { first_index: i, duplicate_index: j })` |
| duplicate path constant name | the lowest index `j` whose `path_constant_name` equals the `path_constant_name` of a declaration at a lower index | `i` is the lowest such lower index | none | `Err(SolidityRecordDeclarationsTryNewErrorReturn::DuplicatePathConstantName { first_index: i, duplicate_index: j })` |
| admitted | no branch above | each of the three names is unique within the list | none | `Ok(SolidityRecordDeclarations { declarations })`, the vector moved from the params in its order |

The three duplicate branches run in the order `struct_name`, `decode_function_name`, `path_constant_name`.

## `SoliditySourceText::as_str`

`SoliditySourceText::as_str(&self) -> &str` returns the text.

## `render`

`render(deps: &RenderDeps, params: RenderParams, payload: RenderPayload) -> RenderReturn`, the trusted form.

Decision: an exhaustive `match` on `payload.body`, one arm per `SolidityLibraryBody` variant. The header in both arms is `// SPDX-License-Identifier: `, the license's text, and `\n`; `pragma solidity `, `major`, `.`, `minor`, `.`, and `patch` in decimal, `;`, and `\n`; `\n`; and `library `, the library name's text, and ` {\n`. The text closes with `}\n`.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| constants | `payload.body` is `SolidityLibraryBody::Constants(entries)` | the match arm | none | `Ok(RenderSuccessReturn { source: SoliditySourceText { text } })`, the body being one line per entry in payload order |
| records | `payload.body` is `SolidityLibraryBody::Records(declarations)` | the match arm | none | `Ok(RenderSuccessReturn { source: SoliditySourceText { text } })`, the body being each declaration's block in payload order, every block after the first preceded by `\n` |

An entry's line is four spaces, the Solidity type, ` internal constant `, the constant name's text, ` = `, the literal, `;`, and `\n`; by value: `Bytes(bytes)` renders type `bytes` and literal `hex"`, each byte as two lowercase hex digits, and `"` (`hex""` when empty); `Uint256(value)` renders type `uint256` and literal `0x` and each byte of `value.big_endian` as two lowercase hex digits in array order; `Uint64(value)` renders type `uint64` and literal the value in decimal; `Uint16(value)` renders type `uint16` and literal the value in decimal; `Bool(value)` renders type `bool` and literal `true` or `false`; `String(literal)` renders type `string` and literal `"`, the literal's text, and `"`.

A declaration's block, each member's ABI type being `SolidityAbiType::try_new(SolidityAbiTypeConstructorParams { kind })`'s `as_str()`, unpacked irrefutably: `    struct `, the struct name, and ` {\n`; per member in order, eight spaces, the ABI type, a space, the member name, and `;\n`; `    }\n`; `\n`; `    string internal constant `, the path constant name, ` = "`, the path's text, and `";\n`; `\n`; `    function `, the decode function name, `(bytes memory line) internal pure returns (`, the struct name, and ` memory) {\n`; eight spaces, `(`, the members' locals joined by `, `, each the ABI type, ` memory` where `decodes_into_memory()`, a space, and the member name, `) = abi.decode(line, (`, the members' ABI types joined by `, `, and `));\n`; eight spaces, `return `, the struct name, `({`, each member as its name, `: `, and its name, joined by `, `, and `});\n`; and `    }\n`.

Ordering: the header, the body, the closing brace. `deps` is not read; equal params and payload yield equal text; no side effect.

## Integration: own entries

Each entry is named and proven by the private integration block of the same name in `integration_test.rs`.

- `render_produces_the_exact_source_of_a_reference_record_library`: condition `render` is called with `SolidityLibraryBody::Records` over one declaration whose members are `Text`, `FixedBytes32`, and `Unsigned64`; outcome the source is the header, the declaration's block of struct, path constant, and decode function in that order, and the closing brace; variation each slot of the declaration carries a different value, so a swap of struct name, decode function name, path constant name, and path fails; edge that must survive the composition: the block is one text assembled from the real `SolidityAbiType` for every member.
- `render_declares_the_struct_with_one_member_line_per_member_in_order`: condition `render` is called with a declaration whose members cover every `CanonicalFieldKind` under distinct unsorted names; outcome the struct has one line per member in member order, eight spaces, the ABI type `SolidityAbiType::as_str` names for the member's kind, a space, the member name, and `;`; variation the members are unsorted and cover every kind, so a kind mapped to another kind's name or a reordered member fails; edge that must survive the composition: the twenty-byte kind is `bytes20`.
- `render_writes_the_path_constant_as_a_string_internal_constant`: condition `render` is called with a declaration whose path constant name and path differ; outcome the block holds `    string internal constant `, the path constant name, ` = "`, the path's text, and `";` with a newline; variation the name and the path are different texts, so a swap fails; edge that must survive the composition: the line is the same whatever the members' kinds.
- `render_declares_the_decode_function_over_a_bytes_memory_line`: condition `render` is called with a declaration whose decode function name and struct name differ; outcome the block holds `    function `, the decode function name, `(bytes memory line) internal pure returns (`, the struct name, and ` memory) {` with a newline; variation the two names are different texts, so a swap fails; edge that must survive the composition: the signature is the same whatever the members' kinds.
- `render_marks_only_bytes_and_string_locals_as_memory`: condition `render` is called with a declaration whose members cover every `CanonicalFieldKind`; outcome the destructuring line has one local per member in order, the ABI type, ` memory` where `SolidityAbiType::decodes_into_memory` returns `true`, a space, and the member name; variation every kind is present, so a kind marked memory that is not `Text` or `Bytes`, or one that is and is not marked, fails; edge that must survive the composition: the marking comes from `decodes_into_memory` and the type from `as_str` for the same kind.
- `render_decodes_the_line_over_the_members_abi_types_in_member_order`: condition `render` is called with a declaration whose members cover every `CanonicalFieldKind` under unsorted names; outcome the destructuring line ends in `) = abi.decode(line, (` the members' ABI types joined by `, ` and `));`; variation unsorted members of every kind, so a reordered or mis-mapped list fails; edge that must survive the composition: the list and the locals name the same type for each member.
- `render_returns_the_struct_built_with_named_arguments`: condition `render` is called with a declaration whose members are named `zeta`, `alpha`, and `mid`; outcome the block ends with eight spaces, `return `, the struct name, `({`, each member as its name, `: `, and its name joined by `, `, and `});` with a newline; variation unsorted member names, so a reordered or mismatched argument fails; edge that must survive the composition: the arguments follow the members whatever their kinds.
- `render_writes_the_first_record_block_without_a_leading_separator`: condition `render` is called with one declaration; outcome the library declaration's line is immediately followed by `    struct `; variation one declaration, so a separator written before the first block fails; edge that must survive the composition: the separator is written between blocks only.
- `render_separates_record_declarations_by_one_empty_line`: condition `render` is called with two declarations; outcome the first block's closing `    }` and newline, one empty line, and the second block's `    struct ` follow in that order, with no second empty line; variation two declarations with different struct names, so a missing or doubled separator fails; edge that must survive the composition: the second block is written exactly as the first.
- `render_writes_record_declarations_in_payload_order`: condition `render` is called with two declarations whose struct names are not in sorted order; outcome the first declaration's block precedes the second's; variation `Beta` then `Alpha`, so a sorted output fails; edge that must survive the composition: the order is the payload's.

## Integration: private surface

The chain of real functions `render` over `SolidityLibraryBody::Records`, `SolidityAbiType::try_new`, `SolidityAbiType::as_str`, and `SolidityAbiType::decodes_into_memory`; no outer-edge collaborator is mocked; the observable result is the `RenderReturn` and its source text; the entries proven are the own entries above.

