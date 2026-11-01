# `render` — interaction spec

Branch contract for the `render` module of the `solidity` module of the `generate` module of `apps/harness-crypto`: `render` renders one Solidity library's source, opening with the license identifier and the exact compiler version pragma its params carry, from a library name and either an ordered list of named constant entries or an ordered list of vector record declarations, each a struct, its decode function, and its vector file's path constant. Each function's branches state condition, decision, dependency call, and the exact return outcome.

## `SolidityLibraryName::try_new`

`try_new(params: SolidityLibraryNameConstructorParams) -> SolidityLibraryNameTryNewReturn`, decided over `params.text.as_bytes()`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| empty | `params.text` has no byte | none | none | `Err(SolidityLibraryNameTryNewErrorReturn::Empty)` |
| leading byte | the byte at index `0` is outside `b'A'..=b'Z'` | the leading byte | none | `Err(SolidityLibraryNameTryNewErrorReturn::LeadingCharacter { byte })` |
| later byte | the lowest index `i` from `1` whose byte is outside `b'A'..=b'Z'`, `b'a'..=b'z'`, and `b'0'..=b'9'` | that byte | none | `Err(SolidityLibraryNameTryNewErrorReturn::InvalidCharacter { index: i, byte })` |
| admitted | the leading byte and every later byte are in their sets | none | none | `Ok(SolidityLibraryName { text })`, the text moved from the params |

## `SolidityConstantName::try_new`

`try_new(params: SolidityConstantNameConstructorParams) -> SolidityConstantNameTryNewReturn`: the same branches in the same order as `SolidityLibraryName::try_new` with `SolidityConstantNameTryNewErrorReturn`'s variants, the leading set `b'A'..=b'Z'` and the later-byte set `b'A'..=b'Z'`, `b'0'..=b'9'`, and `b'_'`; admitted, outcome `Ok(SolidityConstantName { text })`.

## `SpdxLicenseIdentifier::try_new`

`try_new(params: SpdxLicenseIdentifierConstructorParams) -> SpdxLicenseIdentifierTryNewReturn`: the identifier part is the text without its last byte when that byte is `b'+'`, and the whole text otherwise.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| empty | the identifier part has no byte | none | none | `Err(SpdxLicenseIdentifierTryNewErrorReturn::Empty)` |
| invalid | the lowest index `i` of the identifier part whose byte is outside `b'A'..=b'Z'`, `b'a'..=b'z'`, `b'0'..=b'9'`, `b'.'`, and `b'-'` | that byte | none | `Err(SpdxLicenseIdentifierTryNewErrorReturn::InvalidCharacter { index: i, byte })` |
| admitted | every byte of the identifier part is in the set | none | none | `Ok(SpdxLicenseIdentifier { text })`, the whole text |

## `SolidityStringLiteral::try_new`

`try_new(params: SolidityStringLiteralConstructorParams) -> SolidityStringLiteralTryNewReturn`, decided over `params.text.as_bytes()`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| invalid | the lowest index `i` whose byte is outside `0x20..=0x7E` or equals `b'"'` or `b'\\'` | that byte | none | `Err(SolidityStringLiteralTryNewErrorReturn::InvalidCharacter { index: i, byte })` |
| admitted | every byte is in the set | none | none | `Ok(SolidityStringLiteral { text })` |

## `SolidityUint256::try_new`

`try_new(params: SolidityUint256ConstructorParams) -> SolidityUint256TryNewReturn`: one branch; outcome `Ok(SolidityUint256 { big_endian })`, the array moved from the params.

## `SolidityLibraryEntries::try_new`

`try_new(params: SolidityLibraryEntriesConstructorParams) -> SolidityLibraryEntriesTryNewReturn`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| empty | `params.entries` is empty | the length | none | `Err(SolidityLibraryEntriesTryNewErrorReturn::Empty)` |
| duplicate | the lowest index `j` whose entry's `name` equals the `name` of an entry at a lower index, `i` being the lowest such lower index | the name equality | none | `Err(SolidityLibraryEntriesTryNewErrorReturn::DuplicateName { first_index: i, duplicate_index: j })` |
| admitted | every name is unique | none | none | `Ok(SolidityLibraryEntries { entries })`, the vector moved from the params in its order |

## `SolidityTypeName::try_new`

`try_new(params: SolidityTypeNameConstructorParams) -> SolidityTypeNameTryNewReturn`: the same branches in the same order as `SolidityLibraryName::try_new` with `SolidityTypeNameTryNewErrorReturn`'s variants, the leading set `b'A'..=b'Z'` and the later-byte set `b'A'..=b'Z'`, `b'a'..=b'z'`, and `b'0'..=b'9'`; admitted, outcome `Ok(SolidityTypeName { text })`.

## `SolidityMemberName::try_new`

`try_new(params: SolidityMemberNameConstructorParams) -> SolidityMemberNameTryNewReturn`: the same branches in the same order as `SolidityLibraryName::try_new` with `SolidityMemberNameTryNewErrorReturn`'s variants, the leading set `b'a'..=b'z'` and the later-byte set `b'A'..=b'Z'`, `b'a'..=b'z'`, and `b'0'..=b'9'`; admitted, outcome `Ok(SolidityMemberName { text })`.

## `SolidityAbiType::try_new`

`try_new(params: SolidityAbiTypeConstructorParams) -> SolidityAbiTypeTryNewReturn`, decided over `params.name.as_bytes()`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| empty | `params.name` has no byte | none | none | `Err(SolidityAbiTypeTryNewErrorReturn::Empty)` |
| leading byte | the byte at index `0` is outside `b'A'..=b'Z'` and `b'a'..=b'z'` | the leading byte | none | `Err(SolidityAbiTypeTryNewErrorReturn::LeadingCharacter { byte })` |
| later byte | the lowest index `i` from `1` whose byte is outside `b'A'..=b'Z'`, `b'a'..=b'z'`, and `b'0'..=b'9'` | that byte | none | `Err(SolidityAbiTypeTryNewErrorReturn::InvalidCharacter { index: i, byte })` |
| admitted | the leading byte and every later byte are in their sets | none | none | `Ok(SolidityAbiType { text })`, the name moved from the params |

No dependency call and no table: the module holds no mapping from a canonical field kind to an ABI type name; the caller supplies the name.

## `SolidityAbiType::as_str`

`as_str(&self) -> &str`: one branch; outcome the held text.

## `SolidityRecordMember::decodes_into_memory`

`decodes_into_memory(&self) -> bool`: an exhaustive `match` on `self.kind`, `CanonicalFieldKind::Text` and `CanonicalFieldKind::Bytes` to `true` and every other kind to `false`.

## `SolidityRecordMembers::try_new`

`try_new(params: SolidityRecordMembersConstructorParams) -> SolidityRecordMembersTryNewReturn`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| too few | `params.members` holds fewer than two members | the length | none | `Err(SolidityRecordMembersTryNewErrorReturn::TooFew { actual })`, `actual` the length |
| reserved | the lowest index `k` whose member's name text equals `line` | the name text | none | `Err(SolidityRecordMembersTryNewErrorReturn::ReservedName { index: k })` |
| duplicate | the lowest index `j` whose member's `name` equals the `name` of a member at a lower index, `i` being the lowest such lower index | the name equality | none | `Err(SolidityRecordMembersTryNewErrorReturn::DuplicateName { first_index: i, duplicate_index: j })` |
| admitted | at least two members, none named `line`, every name unique | none | none | `Ok(SolidityRecordMembers { members })`, the vector moved in its order |

## `SolidityRecordDeclarations::try_new`

`try_new(params: SolidityRecordDeclarationsConstructorParams) -> SolidityRecordDeclarationsTryNewReturn`, the branches decided in this order, each duplicate check by the lowest index `j` whose name equals the same name of a declaration at a lower index, `i` being the lowest such lower index:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| empty | `params.declarations` is empty | the length | none | `Err(SolidityRecordDeclarationsTryNewErrorReturn::Empty)` |
| duplicate struct name | the lowest `j` whose `struct_name` repeats | the name equality | none | `Err(SolidityRecordDeclarationsTryNewErrorReturn::DuplicateStructName { first_index: i, duplicate_index: j })` |
| duplicate decode function name | the lowest `j` whose `decode_function_name` repeats | the name equality | none | `Err(SolidityRecordDeclarationsTryNewErrorReturn::DuplicateDecodeFunctionName { first_index: i, duplicate_index: j })` |
| duplicate path constant name | the lowest `j` whose `path_constant_name` repeats | the name equality | none | `Err(SolidityRecordDeclarationsTryNewErrorReturn::DuplicatePathConstantName { first_index: i, duplicate_index: j })` |
| admitted | the three names are each unique within the list | none | none | `Ok(SolidityRecordDeclarations { declarations })`, the vector moved in its order |

## `SoliditySourceText::as_str`

`as_str(&self) -> &str` returns the text.

## `render`

`render(deps: &RenderDeps, params: RenderParams, payload: RenderPayload) -> RenderReturn`, the trusted form; dependency call none. The header in both branches is `// SPDX-License-Identifier: `, the license's text, and `\n`; `pragma solidity `, `major`, `.`, `minor`, `.`, and `patch` in decimal, `;`, and `\n`; `\n`; and `library `, the library name's text, and ` {\n`; the text closes with `}\n`.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| constants | `payload.body` is `SolidityLibraryBody::Constants(entries)` | an exhaustive `match` on the body | none | `Ok(RenderSuccessReturn { source: SoliditySourceText { text } })`, the body being one line per entry in payload order |
| records | `payload.body` is `SolidityLibraryBody::Records(declarations)` | the same `match` | none | `Ok(RenderSuccessReturn { source: SoliditySourceText { text } })`, the body being each declaration's block in payload order, every block after the first preceded by `\n` |

### An entry's line

Four spaces, the Solidity type, ` internal constant `, the constant name's text, ` = `, the literal, `;`, and `\n`; by value:

- `Bytes(bytes)`: type `bytes`, literal `hex"`, each byte as two lowercase hex digits, and `"`; `hex""` when empty
- `Uint256(value)`: type `uint256`, literal `0x` and each byte of `value.big_endian` as two lowercase hex digits in array order
- `Uint64(value)`: type `uint64`, literal the value in decimal
- `Uint16(value)`: type `uint16`, literal the value in decimal
- `Bool(value)`: type `bool`, literal `true` or `false`
- `String(literal)`: type `string`, literal `"`, the literal's text, and `"`

### A declaration's block

Each member's ABI type is the `as_str()` of the `abi_type` the member carries, and `decodes_into_memory()` is read from the member:

- `    struct `, the struct name, and ` {\n`
- per member in order, eight spaces, the ABI type, a space, the member name, and `;\n`
- `    }\n`
- `\n`
- `    string internal constant `, the path constant name, ` = "`, the path's text, and `";\n`
- `\n`
- `    function `, the decode function name, `(bytes memory line) internal pure returns (`, the struct name, and ` memory) {\n`
- eight spaces, `(`, the members' locals joined by `, `, each the ABI type, ` memory` where `decodes_into_memory()`, a space, and the member name, `) = abi.decode(line, (`, the members' ABI types joined by `, `, and `));\n`
- eight spaces, `return `, the struct name, `({`, each member as its name, `: `, and its name, joined by `, `, and `});\n`
- `    }\n`

### Ordering

The header, the body, the closing brace; `deps` is not read; equal params and payload yield equal text; no side effect.

## Integration: callee dispositions

No `encoding/factory` entry and no other dependency entry has a route that reaches a function this node authors or its tests run; the module reads `encoding`'s `CanonicalFieldKind` as a closed enum and calls no dependency function.

## Integration: own entries

Each entry is proven by the private integration block of the same name in `integration_test.rs`.

- `render_produces_the_exact_source_of_a_reference_record_library`: condition `render` is called with `SolidityLibraryBody::Records` over one declaration whose members are `Text`, `FixedBytes32`, and `Unsigned64`, each carrying the ABI type the arrangement supplies; outcome the source is the header, the declaration's block of struct, path constant, and decode function in that order, and the closing brace; variation each slot of the declaration carries a different value, so a swap of struct name, decode function name, path constant name, and path fails; edge that must survive the composition: the block is one text assembled from the ABI type each member carries.
- `render_declares_the_struct_with_one_member_line_per_member_in_order`: condition `render` is called with a declaration whose members cover every `CanonicalFieldKind` under distinct unsorted names, each carrying a distinct ABI type; outcome the struct has one line per member in member order, eight spaces, the text of the member's `abi_type`, a space, the member name, and `;`; variation the members are unsorted, cover every kind, and carry types that differ from one another, so a type taken from the kind, a type swapped between members, or a reordered member fails; edge that must survive the composition: the type written is the type the member carries and no other, whatever its kind.
- `render_writes_the_path_constant_as_a_string_internal_constant`: condition `render` is called with a declaration whose path constant name and path differ; outcome the block holds `    string internal constant `, the path constant name, ` = "`, the path's text, and `";` with a newline; variation the name and the path are different texts, so a swap fails; edge that must survive the composition: the line is the same whatever the members' kinds.
- `render_declares_the_decode_function_over_a_bytes_memory_line`: condition `render` is called with a declaration whose decode function name and struct name differ; outcome the block holds `    function `, the decode function name, `(bytes memory line) internal pure returns (`, the struct name, and ` memory) {` with a newline; variation the two names are different texts, so a swap fails; edge that must survive the composition: the signature is the same whatever the members' kinds.
- `render_marks_only_bytes_and_string_locals_as_memory`: condition `render` is called with a declaration whose members cover every `CanonicalFieldKind`, each carrying a type the arrangement supplies; outcome the destructuring line has one local per member in order, the member's ABI type, ` memory` where `SolidityRecordMember::decodes_into_memory` returns `true`, a space, and the member name; variation every kind is present, so a kind marked memory that is not `Text` or `Bytes`, or one that is and is not marked, fails; edge that must survive the composition: the marking comes from the member's kind and the type from the member's `abi_type`.
- `render_decodes_the_line_over_the_members_abi_types_in_member_order`: condition `render` is called with a declaration whose members cover every `CanonicalFieldKind` under unsorted names, each carrying a distinct type; outcome the destructuring line ends in `) = abi.decode(line, (` the members' ABI types joined by `, ` and `));`; variation unsorted members of every kind, so a reordered or mis-assigned list fails; edge that must survive the composition: the list and the locals name the same type for each member.
- `render_returns_the_struct_built_with_named_arguments`: condition `render` is called with a declaration whose members are named `zeta`, `alpha`, and `mid`; outcome the block ends with eight spaces, `return `, the struct name, `({`, each member as its name, `: `, and its name joined by `, `, and `});` with a newline; variation unsorted member names, so a reordered or mismatched argument fails; edge that must survive the composition: the arguments follow the members whatever their kinds.
- `render_writes_the_first_record_block_without_a_leading_separator`: condition `render` is called with one declaration; outcome the library declaration's line is immediately followed by `    struct `; variation one declaration, so a separator written before the first block fails; edge that must survive the composition: the separator is written between blocks only.
- `render_separates_record_declarations_by_one_empty_line`: condition `render` is called with two declarations; outcome the first block's closing `    }` and newline, one empty line, and the second block's `    struct ` follow in that order, with no second empty line; variation two declarations with different struct names, so a missing or doubled separator fails; edge that must survive the composition: the second block is written exactly as the first.
- `render_writes_record_declarations_in_payload_order`: condition `render` is called with two declarations whose struct names are not in sorted order; outcome the first declaration's block precedes the second's; variation `Beta` then `Alpha`, so a sorted output fails; edge that must survive the composition: the order is the payload's.

## Integration: private surface

The chain of real functions `render` over `SolidityLibraryBody::Records`, `SolidityAbiType::as_str`, and `SolidityRecordMember::decodes_into_memory`; no outer-edge collaborator is mocked; the observable result is the `RenderReturn` and its source text; the entries proven are the own entries above.
