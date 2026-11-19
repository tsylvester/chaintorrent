# `settings` — interaction spec

Branch contract for the `settings` module of the `solidity` module of `apps/harness-crypto`: `settings` owns the Solidity concrete's admitted value types — the file name and the generated directory the emitted paths are built from, the per-kind and per-library name sets, the settings the concrete carries, and the member's settings document — its `try_new` constructors admitting each name text, its `try_from` guard narrowing the member's untrusted document, and `admit_solidity_generate_settings` resolving each supplied or defaulted setting text against the declaration the caller hands it into `SolidityGenerateSettings`. Each function's branches state condition, decision, dependency call, and the exact return outcome.

## `GeneratedFileName::try_new`

`GeneratedFileName::try_new(params: GeneratedFileNameConstructorParams) -> GeneratedFileNameTryNewReturn`, decided over `params.text.as_bytes()`; `as_str(&self) -> &str` returns the admitted text.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| empty | `params.text.as_bytes()` holds no byte | none | none | `Err(GeneratedFileNameTryNewErrorReturn::Empty)` |
| invalid | the lowest index `i` whose byte is outside `b'A'..=b'Z'`, `b'a'..=b'z'`, `b'0'..=b'9'`, `b'.'`, `b'_'`, and `b'-'` | none | none | `Err(GeneratedFileNameTryNewErrorReturn::InvalidCharacter { index: i, byte })` |
| reserved | the text is `.` or `..` | none | none | `Err(GeneratedFileNameTryNewErrorReturn::Reserved)` |
| admitted | none of the above | none | none | `Ok(GeneratedFileName { text })` |

## `GeneratedDirectory::try_new`

`GeneratedDirectory::try_new(params: GeneratedDirectoryConstructorParams) -> GeneratedDirectoryTryNewReturn`; `segments(&self) -> &[GeneratedFileName]` returns the segments in the order `try_new` received them, one branch, outcome the slice.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| empty | `params.segments` is empty | none | none | `Err(GeneratedDirectoryTryNewErrorReturn::Empty)` |
| admitted | `params.segments` is nonempty | none | none | `Ok(GeneratedDirectory { segments })` |

## `SolidityGenerateSettingsDocument::try_from`

`SolidityGenerateSettingsDocument::try_from(value: serde_json::Value) -> SolidityGenerateSettingsDocumentTryFromReturn`, the guard on entry; a key's place is its order in the table's key order, and a `kinds` entry's key is any text, the admission deciding whether it names a declared kind.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| not a table | the value is not an object | none | none | `Err(NotATable { path: "" })` |
| unknown key | the lowest key, in key order, outside `kinds`, `constants_library`, `records_library`, `foundry_root`, `generated_directory`, `compiler_version`, and `license` | none | none | `Err(UnknownKey { path: "", key })` |
| kinds not a table | a present `kinds` value is not an object | none | none | `Err(NotATable { path: "kinds" })` |
| kind entry not a table | an entry of `kinds`, in key order, is not an object | none | none | `Err(NotATable { path: "kinds.<key>" })` |
| unknown kind key | the lowest key of a kind's table, in key order, outside `file`, `struct_name`, `decode_function`, and `path_constant` | none | none | `Err(UnknownKey { path: "kinds.<key>", key })` |
| kind field not text | a present value of a kind's table is not a string | none | none | `Err(NotText { path: "kinds.<key>.<field>" })` |
| library not a table | a present `constants_library` or `records_library` value is not an object | none | none | `Err(NotATable { path })` |
| unknown library key | a key of a library's table outside `file` and `library` | none | none | `Err(UnknownKey { path, key })` |
| library field not text | a present `file` or `library` value is not a string | none | none | `Err(NotText { path: "<library>.<field>" })` |
| text setting not text | a present `foundry_root`, `compiler_version`, or `license` value is not a string | none | none | `Err(NotText { path })` |
| list setting not a text list | a present `generated_directory` value is not an array of strings | none | none | `Err(NotTextList { path: "generated_directory" })` |
| narrowed | none of the above | none | none | `Ok` of the document holding each present value and `None` for each absent one |

## `admit_solidity_generate_settings`

`admit_solidity_generate_settings(deps: &AdmitSolidityGenerateSettingsDeps, params: AdmitSolidityGenerateSettingsParams, payload: AdmitSolidityGenerateSettingsPayload) -> AdmitSolidityGenerateSettingsReturn`, the trusted form; the payload arrives narrowed by the guard; `params` is not read; no dependency call other than the constructors named below.

Resolving a setting's text, used by every branch below and itself making no dependency call: the statement is the first of `deps.declaration.settings` whose `name` equals the setting's dotted path — absent, `Err(UndeclaredSetting { setting })`; `Fixed { value }` with a supplied text, `Err(FixedSetting { setting, supplied })`; `Fixed { value }` without one, the text is `value`; `Settable { bounds, default }` with a supplied text, the text is that text and without one it is `default`; the bounds of a version setting are read by the compiler-version branch.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| unknown document kind | the lowest key of `payload.document.kinds`, in key order, whose text equals the `name` of no kind of `deps.declaration.kinds` | none | none | `Err(UnknownKind { text })`, decided before any other branch |
| undeclared kind | the lowest kind of `deps.declaration.kinds`, in declared order, whose `name` equals the `Debug` text of no member of `SOLIDITY_VECTOR_KINDS` | none | none | `Err(UnknownKind { text })` with the kind's `name`, decided before any setting is read; else the kind's `VectorKind` is that member |
| declared kind | per declared kind in declared order, the names resolved in the order `file`, `struct_name`, `decode_function`, `path_constant` from the setting paths `kinds.<kind>.<field>`, the supplied text read from the document entry named by the kind's `name`, `<kind>` in the setting paths being that `name` | each refusal returns | `GeneratedFileName::try_new`, `SolidityTypeName::try_new`, `SolidityMemberName::try_new`, `SolidityConstantName::try_new`, in the same order | a refusal `Err(FileName { setting, error })`, `Err(TypeName { setting, error })`, `Err(MemberName { setting, error })`, or `Err(ConstantName { setting, error })`; else one `SolidityVectorKindSettings { kind, names }` |
| constants library | the `constants_library` `file` text resolved, then the `library` text resolved | each refusal returns | `GeneratedFileName::try_new`, then `SolidityLibraryName::try_new` | a refusal `Err(FileName { setting, error })` or `Err(LibraryName { setting, error })`; else a `SolidityLibraryNames` |
| records library | the `records_library` `file` text resolved, then the `library` text resolved | each refusal returns | `GeneratedFileName::try_new`, then `SolidityLibraryName::try_new` | a refusal `Err(FileName { setting, error })` or `Err(LibraryName { setting, error })`; else a `SolidityLibraryNames` |
| Foundry root | the `foundry_root` text resolved | none | none | `PathBuf::from` of the text |
| generated directory | the document supplies a list, its texts are the segments, and it does not, the resolved default text split on `/` are the segments; a supplied list is not resolved against the statement's bounds, the statement being unbounded | each segment admitted in order, the lowest refusing index returning; the statement is read for existence, so a declaration without it returns `Err(UndeclaredSetting { setting: "generated_directory" })` | `GeneratedFileName::try_new` per segment, then `GeneratedDirectory::try_new` | a segment refusal `Err(DirectorySegment { index, error })`; a directory refusal `Err(Directory { error })` |
| compiler version | the `compiler_version` text resolved, split on `.` | the bounds read by an exhaustive `match` on the statement's bounds: `Unbounded`, the release is admitted; `VersionWithin { at_least, below }`, the release less than `at_least` or not less than `below` by major, then minor, then patch | none | not three parts, `Err(CompilerVersionParts { setting, text })`; the lowest index whose part `u16::from_str` refuses, `Err(CompilerVersionPart { setting, index, text: part })`; out of bounds, `Err(OutOfBounds { setting, supplied: text })`; else `SolidityCompilerVersion { major, minor, patch }` |
| license | the `license` text resolved | none | `SpdxLicenseIdentifier::try_new` | a refusal `Err(License { error })` |
| computed | every branch above admitted | none | none | `Ok(AdmitSolidityGenerateSettingsSuccessReturn { settings })` |

### Ordering

The kinds table, each declared kind, the constants library, the records library, the Foundry root, the generated directory, the compiler version, then the license; the first refusal returns and no later value is read; no side effect.

## Integration: callee dispositions

- `harness-crypto/generate/solidity/render`, `harness-crypto/generate/solidity/group_vectors`, `encoding/factory`, and `chain/create_chain_forms`: no entry of theirs passes through a function of this node or ends at one of its tests, this node calling none of their functions beyond the `render` name constructors, whose entries are proven in `render`'s own node.

## Integration: own entries

- `the_declared_defaults_are_admitted_by_the_real_constructors`: condition `admit_solidity_generate_settings` is called over `SOLIDITY_GENERATE_DECLARATION` with a document that supplies nothing; outcome `Ok` whose settings hold one `SolidityVectorKindSettings` per declared kind in declared order, the constants and records libraries named `Constants.sol` with `Constants` and `VectorRecords.sol` with `VectorRecords`, the Foundry root `contracts/evm`, the generated directory the one segment `generated`, the compiler version `0.8.38`, and the license `UNLICENSED`; variation every kind of the declaration, whose defaults differ from one another, so a default of one kind used for another, or a default no real constructor admits, fails; edge that must survive the composition: each name is the real `render` constructor's admitted value for its text.
- `the_default_names_are_distinct_across_the_kinds_and_the_libraries`: condition the same call; outcome no two kinds share a file name, struct name, decode function name, or path constant name, and the constants library and the records library share neither file name nor library name; variation every declared kind and both libraries, so a default repeated between two of them fails; edge that must survive the composition: the names are the admitted types' values, compared by their real equality.

## Integration: private surface

The chain of real functions `admit_solidity_generate_settings`, `GeneratedFileName::try_new`, `GeneratedDirectory::try_new`, and the `render` name constructors `SolidityTypeName::try_new`, `SolidityMemberName::try_new`, `SolidityConstantName::try_new`, `SolidityLibraryName::try_new`, and `SpdxLicenseIdentifier::try_new`; no outer-edge collaborator is mocked; the observable result is the `AdmitSolidityGenerateSettingsReturn` and the admitted types; the entries proven are the own entries above.

## Integration: public surface

None; this node's boundary is not a public entry, so it carries no public integration test.
