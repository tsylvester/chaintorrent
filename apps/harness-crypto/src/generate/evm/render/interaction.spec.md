# render interactions

`render` assembles the source text of one Solidity library from a license
identifier, a compiler version, a library name, and either a list of named
constant entries or a list of vector record declarations. It reads no
dependency, draws no randomness, reads no clock, and touches no filesystem;
every refusal is a constructor's. The validated name, literal, value, entry,
member, and declaration types admit only text and lists the rendering can
emit, so the function itself has no failure branch.

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

Decision: the same branches in the same order as `SolidityLibraryName::try_new`
with `SolidityConstantNameTryNewErrorReturn`'s variants, the later-byte set
being `b'A'..=b'Z'`, `b'0'..=b'9'`, and `b'_'`; admitted, outcome
`Ok(SolidityConstantName { text })`.

## `SpdxLicenseIdentifier::try_new`

`SpdxLicenseIdentifier::try_new(params: SpdxLicenseIdentifierConstructorParams) -> SpdxLicenseIdentifierTryNewReturn`

Decision: the identifier part is the text without its last byte when that byte
is `b'+'`, and the whole text otherwise; branches are decided over the
identifier part, indexes into the whole text.

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

Decision: the same branches in the same order as `SolidityLibraryName::try_new`
with `SolidityTypeNameTryNewErrorReturn`'s variants, the leading set
`b'A'..=b'Z'` and the later-byte set `b'A'..=b'Z'`, `b'a'..=b'z'`, and
`b'0'..=b'9'`; admitted, outcome `Ok(SolidityTypeName { text })`.

## `SolidityMemberName::try_new`

`SolidityMemberName::try_new(params: SolidityMemberNameConstructorParams) -> SolidityMemberNameTryNewReturn`

Decision: the same branches in the same order as `SolidityLibraryName::try_new`
with `SolidityMemberNameTryNewErrorReturn`'s variants, the leading set
`b'a'..=b'z'` and the later-byte set `b'A'..=b'Z'`, `b'a'..=b'z'`, and
`b'0'..=b'9'`; admitted, outcome `Ok(SolidityMemberName { text })`.

## `SolidityAbiType::try_new`

`SolidityAbiType::try_new(params: SolidityAbiTypeConstructorParams) -> SolidityAbiTypeTryNewReturn`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| admitted | always | one branch | none | `Ok(SolidityAbiType { kind })`, the kind moved from the params |

## `SolidityAbiType::as_str`

`SolidityAbiType::as_str(&self) -> &'static str`

Decision: an exhaustive `match` on `self.kind`, one arm per
`CanonicalFieldKind` variant, so a variant with no arm fails to compile:
`FixedBytes32` to `bytes32`, `Unsigned16` to `uint16`, `Unsigned32` to
`uint32`, `Unsigned64` to `uint64`, `Text` to `string`, `Bytes` to `bytes`,
`FixedBytes20` to `bytes20`, and `Unsigned256` to `uint256`.

## `SolidityAbiType::decodes_into_memory`

`SolidityAbiType::decodes_into_memory(&self) -> bool`

Decision: an exhaustive `match` on `self.kind`, `Text` and `Bytes` to `true`
and every other kind to `false`.

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

The three duplicate branches run in the order `struct_name`,
`decode_function_name`, `path_constant_name`.

## `SoliditySourceText::as_str`

`SoliditySourceText::as_str(&self) -> &str` returns the text.

## `render`

`render(deps: &RenderDeps, params: RenderParams, payload: RenderPayload) -> RenderReturn`, the trusted form.

Decision: an exhaustive `match` on `payload.body`, one arm per
`SolidityLibraryBody` variant. The header in both arms is
`// SPDX-License-Identifier: `, the license's text, and `\n`; `pragma solidity `,
`major`, `.`, `minor`, `.`, and `patch` in decimal, `;`, and `\n`; `\n`; and
`library `, the library name's text, and ` {\n`. The text closes with `}\n`.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| constants | `payload.body` is `SolidityLibraryBody::Constants(entries)` | the match arm | none | `Ok(RenderSuccessReturn { source: SoliditySourceText { text } })`, the body being one line per entry in payload order |
| records | `payload.body` is `SolidityLibraryBody::Records(declarations)` | the match arm | none | `Ok(RenderSuccessReturn { source: SoliditySourceText { text } })`, the body being each declaration's block in payload order, every block after the first preceded by `\n` |

An entry's line is four spaces, the Solidity type, ` internal constant `, the
constant name's text, ` = `, the literal, `;`, and `\n`; by value:
`Bytes(bytes)` renders type `bytes` and literal `hex"`, each byte as two
lowercase hex digits, and `"` (`hex""` when empty); `Uint256(value)` renders
type `uint256` and literal `0x` and each byte of `value.big_endian` as two
lowercase hex digits in array order; `Uint64(value)` renders type `uint64`
and literal the value in decimal; `Uint16(value)` renders type `uint16` and
literal the value in decimal; `Bool(value)` renders type `bool` and literal
`true` or `false`; `String(literal)` renders type `string` and literal `"`,
the literal's text, and `"`.

A declaration's block, each member's ABI type being
`SolidityAbiType::try_new(SolidityAbiTypeConstructorParams { kind })`'s
`as_str()`, unpacked irrefutably: `    struct `, the struct name, and ` {\n`;
per member in order, eight spaces, the ABI type, a space, the member name,
and `;\n`; `    }\n`; `\n`; `    string internal constant `, the path
constant name, ` = "`, the path's text, and `";\n`; `\n`; `    function `,
the decode function name, `(bytes memory line) internal pure returns (`, the
struct name, and ` memory) {\n`; eight spaces, `(`, the members' locals
joined by `, `, each the ABI type, ` memory` where `decodes_into_memory()`,
a space, and the member name, `) = abi.decode(line, (`, the members' ABI
types joined by `, `, and `));\n`; eight spaces, `return `, the struct name,
`({`, each member as its name, `: `, and its name, joined by `, `, and
`});\n`; and `    }\n`.

Ordering: the header, the body, the closing brace. `deps` is not read; equal
params and payload yield equal text; no side effect.
