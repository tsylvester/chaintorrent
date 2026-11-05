# `admit_generate` — interaction spec

Branch contract for the `admit_generate` module of `apps/harness-crypto`: `admit_generate` turns the document's `generate` table into the types the generate family and its concrete declare — the `concrete` text mapped to the member of `GENERATE_CONCRETES` it names, each key of `kinds` mapped to the kind of the `SOLIDITY_GENERATE_DECLARATION` entry it names and the entry's quantities and names admitted through their constructors, the two libraries, the generated directory, each curve directory keyed by the member of `PAIRING_CURVES` its key names, the compiler version, and the license — and refuses the first text or quantity no declared set names and no constructor admits, every callee refusal carried whole with the document path that held the text. Each branch states condition, decision, dependency call, and the exact return outcome.

## `admit_generate`

`admit_generate(deps: &AdmitGenerateDeps, params: AdmitGenerateParams, payload: AdmitGeneratePayload) -> AdmitGenerateReturn`, the trusted form: the document's `generate` table arrives as data in `payload`; the function reads no file, no environment variable, and no clock; `deps` and `params` carry nothing and are not read.

### Naming

Private `member_named`: given a text, a slice of members, and a function from a member to the value it is named by; decision the first member in declared order for which `format!("{:?}", &name_of(member))` equals the text, compared whole and case-sensitively; outcome that member, or none; dependency call none — each declared set is a constant the family declares, and a mock selection is never a member of a declared set, so its text names no member.

### Version parsing

Private `parse_compiler_version`: given a text; decision the text split on `.`; a split that does not yield three parts returns `Err(CompilerVersionErrorReturn::Parts { count })`, `count` the number of parts; a part `u16::from_str` refuses returns `Err(CompilerVersionErrorReturn::Part { index, text })`, the lowest such index; otherwise outcome `SolidityCompilerVersion { major, minor, patch }` from the three parts in order.

### Concrete refused

Condition `member_named` over `GENERATE_CONCRETES` and `payload.generate.concrete` returns none; outcome `Err(AdmitGenerateErrorReturn::Concrete { text })`, `text` the refused text.

### Concrete admitted

Condition a member returns; outcome that `GenerateConcreteKind`, which the success branch's exhaustive `match` reads.

### Kind refused

Per entry of `payload.generate.kinds` in key order: condition `member_named` over `SOLIDITY_GENERATE_DECLARATION.kinds`, each declaration named by its `kind`, and the entry's key returns none; outcome `Err(AdmitGenerateErrorReturn::Kind { text })`, `text` the entry's key.

### Kind admitted

Condition a member returns; outcome that declaration's `kind`, which the entry's `SolidityVectorKindSettings` and `VectorKindConfiguration` carry.

### Count refused, admitted, or absent

Per entry, condition the entry's `count` is `Some`; dependency call `VectorCount::try_new(VectorCountConstructorParams { count })`; a refusal returns `Err(AdmitGenerateErrorReturn::Count { kind, error })`, `kind` the entry's key and `error` the refusal unchanged; outcome the `VectorCount`; `None` continues as `None`.

### Terms refused, admitted, or absent

Per entry, condition the entry's `terms` is `Some`; dependency call `MsmTermCount::try_new(MsmTermCountConstructorParams { count })`; a refusal returns `Err(AdmitGenerateErrorReturn::Terms { kind, error })` with the refusal unchanged; outcome the `MsmTermCount`; `None` continues as `None`.

### Message length carried

Per entry, the entry's `message_length` continues as the document holds it; no decision and no dependency call.

### Kind names refused

Per entry, dependency calls `GeneratedFileName::try_new` over `file`, `SolidityTypeName::try_new` over `struct_name`, `SolidityMemberName::try_new` over `decode_function`, and `SolidityConstantName::try_new` over `path_constant`, in that order; a refusal returns `Err` of `File`, `StructName`, `DecodeFunction`, or `PathConstant` with `kind` the entry's key and `error` the refusal unchanged.

### Kind names admitted

Condition every name returns; outcome the entry's `SolidityVectorKindNames` of the admitted names, and then its `SolidityVectorKindSettings { kind, names }` and `VectorKindConfiguration { kind, quantities }`.

### Constants library refused

Condition `constants_library.file` returns none of `GeneratedFileName::try_new`, or `constants_library.library` returns none of `SolidityLibraryName::try_new`, in that order; outcome `Err` of `ConstantsLibraryFile` or `ConstantsLibraryName` with `error` the refusal unchanged.

### Constants library admitted

Condition both return; outcome `SolidityLibraryNames { file_name, library_name }` of the admitted names.

### Records library refused

Condition `records_library.file` returns none of `GeneratedFileName::try_new`, or `records_library.library` returns none of `SolidityLibraryName::try_new`, in that order; outcome `Err` of `RecordsLibraryFile` or `RecordsLibraryName` with `error` the refusal unchanged.

### Records library admitted

Condition both return; outcome `SolidityLibraryNames { file_name, library_name }` of the admitted names.

### Generated directory segment refused

Per segment of `generated_directory` in order: dependency call `GeneratedFileName::try_new`; a refusal returns `Err(AdmitGenerateErrorReturn::GeneratedDirectorySegment { index, error })`, `index` the segment's position and `error` the refusal unchanged.

### Generated directory refused or admitted

After every segment: dependency call `GeneratedDirectory::try_new(GeneratedDirectoryConstructorParams { segments })`; a refusal returns `Err(AdmitGenerateErrorReturn::GeneratedDirectory { error })` with the refusal unchanged; outcome the `GeneratedDirectory` of the admitted segments in order.

### Curve key refused

Per entry of `payload.generate.curve_directories` in key order: condition `member_named` over `PAIRING_CURVES` and the entry's key returns none; outcome `Err(AdmitGenerateErrorReturn::CurveKey { text })`, `text` the entry's key.

### Curve directory refused

Condition a member returns; dependency call `GeneratedFileName::try_new` over the entry's value; a refusal returns `Err(AdmitGenerateErrorReturn::CurveDirectory { curve, error })`, `curve` the named member and `error` the refusal unchanged.

### Curve directory admitted

Condition the value returns; outcome the `PairingCurve` the key names paired with its admitted `GeneratedFileName`.

### Missing curve

After every entry, for each member of `PAIRING_CURVES` in declared order: condition no entry names it; outcome `Err(AdmitGenerateErrorReturn::MissingCurveDirectory { curve })`, `curve` the first such member. Otherwise `SolidityCurveDirectories` is built with its `bn254` field taken from the entry for `PairingCurve::Bn254` and its `bls12_381` field from the entry for `PairingCurve::Bls12381`.

### Compiler version refused

Dependency call the private `parse_compiler_version` over `compiler_version`; a refusal returns `Err(AdmitGenerateErrorReturn::CompilerVersion { text, error })`, `text` the version text and `error` the refusal unchanged.

### Compiler version admitted

Condition it returns; outcome the `SolidityCompilerVersion` of the three parts.

### License refused

Dependency call `SpdxLicenseIdentifier::try_new(SpdxLicenseIdentifierConstructorParams { text })`; a refusal returns `Err(AdmitGenerateErrorReturn::License { error })` with the refusal unchanged.

### License admitted

Condition it returns; outcome the `SpdxLicenseIdentifier`.

### Success

The settings are `SolidityGenerateSettings { kinds, constants_library, records_library, foundry_root, generated_directory, curve_directories, compiler_version, license }` with `foundry_root` cloned from the document; the concrete is an exhaustive `match` on the admitted `GenerateConcreteKind`, its `Solidity` arm `GenerateConcrete::Solidity { settings: Box::new(settings) }`; outcome `Ok(AdmitGenerateSuccessReturn { admitted: AdmittedGenerate { concrete, kinds } })`, `kinds` the admitted `VectorKindConfiguration`s in key order.

### Ordering

Concrete, then each kind in key order with its key, count, terms, file, struct name, decode function, and path constant, then the constants library's file and name, the records library's file and name, the generated directory's segments and whole, the curve directories in key order and then the missing curves in declared order, the compiler version, and the license; the first refusal returns and every later value is unread; no side effect.

### Unreached branches

- A quantity present for a kind whose declared sizing does not take it, and a kind set that omits or repeats a declared kind: the function admits each kind it is given and the map holds each key once, so those refusals are `harness-crypto/generate/factory`'s and have no unit block here.

## Integration: own entries

- `every_declared_kind_is_admitted_to_its_own_settings_and_configuration`: condition `admit_generate` over a document whose `kinds` names one kind of `SOLIDITY_GENERATE_DECLARATION.kinds` with the quantities its declared sizing takes and names distinct from every other kind's, every collaborator real, the declaration and the constructors the generate family declares; outcome `Ok` whose `admitted.kinds` holds the `VectorKindConfiguration` of that kind with the quantities the document holds and whose settings hold that kind's `SolidityVectorKindSettings` with the names the document holds; variation every `VectorKindDeclaration` of the declaration in turn and each sizing it declares, the document's quantities chosen by an exhaustive `match` on the sizing; edge a text matched to a neighbouring kind, a quantity placed in another field, or a name placed in another field differs from the value built in Arrange.
- `every_declared_curve_directory_reaches_the_field_its_curve_names`: condition `admit_generate` over a document whose `curve_directories` names every member of `PAIRING_CURVES`, the member under test with one directory text and every other member with another, every collaborator real; outcome `Ok` whose `SolidityCurveDirectories` holds the member under test's text in the field that member names and the other text in the other field; variation every member of `PAIRING_CURVES` in turn as the member under test; edge the two fields exchanged, a key matched to a neighbouring member, or an entry read by its position instead of its key differs from the directories built in Arrange.
- `an_undeclared_name_is_never_admitted`: condition `admit_generate` over a document whose `concrete` is the `Debug` text of the family's mock selection, and over a document whose curve key is the `Debug` text of the pairing family's mock variant, the other values admitted; outcome `Err` carrying `Concrete` or `CurveKey` and that text; variation the two texts in turn; edge a declared set that held its mock selection would admit it.

## Integration: callee dispositions

No entry of a direct dependency passes through `admit_generate` or ends at its tests, because it calls only the constructors the generate family and its functions declare and reads each declared set as a constant, and no entry of `harness-crypto/generate/solidity/render`, `harness-crypto/generate/solidity/group_vectors`, `harness-crypto/generate/solidity`, `harness-crypto/generate/factory`, or `harness-crypto/config/layers` is proven through those constructors.

## Integration: private surface

The chain of real functions `admit_generate`, `member_named`, `parse_compiler_version`, and each constructor named above, all running for real over the real declared sets; no outer-edge collaborator; the observable result is the `AdmitGenerateReturn`; the entries proven are `every_declared_kind_is_admitted_to_its_own_settings_and_configuration`, `every_declared_curve_directory_reaches_the_field_its_curve_names`, and `an_undeclared_name_is_never_admitted`.

## Integration: public surface

An outside caller invokes `admit_generate` through the crate root with an `AdmitGenerateDeps`, an `AdmitGenerateParams`, and an `AdmitGeneratePayload` holding a `GenerateTable`, and observes the `AdmitGenerateReturn` and the `AdmittedGenerate` in its success; the entries proven are `every_declared_kind_is_admitted_to_its_own_settings_and_configuration`, `every_declared_curve_directory_reaches_the_field_its_curve_names`, and `an_undeclared_name_is_never_admitted`; there is no outer-edge collaborator.
