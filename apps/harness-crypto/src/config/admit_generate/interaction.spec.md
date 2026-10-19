# `admit_generate` — interaction spec

Branch contract for the `admit_generate` module of the `config` module of `apps/harness-crypto`: `admit_generate`, the function the configuration document's generate table is admitted by — the concrete's text mapped to the `GenerateConcrete` variant it names; each vector kind's text mapped to its `VectorKind`, its count and term count admitted through `VectorCount` and `MsmTermCount`, its message length carried, and its file, struct, decode function, and path constant names admitted through `GeneratedFileName` and the render name types; the constants and vector-record libraries' file and library names, the Foundry project root, the generated directory's segments, the curve directories, the Solidity compiler version, and the license identifier each through their own constructors — the kinds admitted into the family's `VectorKindConfiguration`s and the concrete's `EvmVectorKindSettings`, every refusal carried with the field it was read from, and the kind set's admission against the concrete's declaration left to `harness-crypto/generate/factory`. Each branch states condition, decision, dependency call, and the exact return outcome.

## `admit_generate`

`admit_generate(deps: &AdmitGenerateDeps, params: AdmitGenerateParams, payload: AdmitGeneratePayload<'_>) -> AdmitGenerateReturn`: the payload is the document the deserializer proved the shape of, and this function proves its content; `deps` and `params` carry nothing and are not read. `generate` below is `payload.generate`.

### Concrete

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| concrete | none | a `match` on `generate.concrete.as_str()` | none | `"evm"` selects the EVM admission below; any other text `Err(AdmitGenerateErrorReturn::UnknownValue { field: GenerateField::Concrete, text })`, `text` a clone of the text |

### Kinds

In the EVM arm, for each `(index, entry)` of `generate.kinds` in order, each field in this order.

The kind is decided by a `match` on `entry.kind.as_str()`; any other text is refused with outcome `Err(AdmitGenerateErrorReturn::UnknownValue { field: GenerateField::Kind { index }, text })`, `text` a clone of the text:

| Text | `VectorKind` |
|---|---|
| `"g1_addition"` | `G1Addition` |
| `"g1_multiplication"` | `G1Multiplication` |
| `"pairing_check"` | `PairingCheck` |
| `"g2_addition"` | `G2Addition` |
| `"g1_multi_scalar_multiplication"` | `G1MultiScalarMultiplication` |
| `"g2_multi_scalar_multiplication"` | `G2MultiScalarMultiplication` |
| `"decode_rejection"` | `DecodeRejection` |
| `"hash_to_scalar"` | `HashToScalar` |
| `"identity_mapping"` | `IdentityMapping` |
| `"possession"` | `Possession` |
| `"challenge"` | `Challenge` |
| `"mint_proof"` | `MintProof` |
| `"transfer_proof"` | `TransferProof` |

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| kind | `entry.kind` | the text's `match` | none | the `VectorKind` the text names; any other text `Err(AdmitGenerateErrorReturn::UnknownValue { field: GenerateField::Kind { index }, text })`, `text` a clone of the text |
| count | `entry.count` is `None` or `Some(count)` | `None` admits `None`; `Some` the constructor | `VectorCount::try_new(VectorCountConstructorParams { count })` | the `VectorCount` wrapped in `Some`, or `None`; a refusal `Err(AdmitGenerateErrorReturn::VectorCount { index, error })` |
| term count | `entry.terms` is `None` or `Some(count)` | likewise | `MsmTermCount::try_new(MsmTermCountConstructorParams { count })` | the `MsmTermCount` wrapped in `Some`, or `None`; a refusal `Err(AdmitGenerateErrorReturn::MsmTermCount { index, error })` |
| message length | `entry.message_length` | none | none | the `Option<usize>` carried as it is |
| file name | `entry.file_name` | the constructor over a clone of the text | `GeneratedFileName::try_new(GeneratedFileNameConstructorParams { text })` | the `GeneratedFileName`; a refusal `Err(AdmitGenerateErrorReturn::FileName { field: GenerateField::KindFileName { index }, error })` |
| struct name | `entry.struct_name` | the constructor over a clone of the text | `SolidityTypeName::try_new(SolidityTypeNameConstructorParams { text })` | the `SolidityTypeName`; a refusal `Err(AdmitGenerateErrorReturn::StructName { index, error })` |
| decode function name | `entry.decode_function_name` | the constructor over a clone of the text | `SolidityMemberName::try_new(SolidityMemberNameConstructorParams { text })` | the `SolidityMemberName`; a refusal `Err(AdmitGenerateErrorReturn::DecodeFunctionName { index, error })` |
| path constant name | `entry.path_constant_name` | the constructor over a clone of the text | `SolidityConstantName::try_new(SolidityConstantNameConstructorParams { text })` | the `SolidityConstantName`; a refusal `Err(AdmitGenerateErrorReturn::PathConstantName { index, error })` |
| admitted kind | every field of the entry admitted | none | none | `EvmVectorKindSettings { kind, names: EvmVectorKindNames { file_name, struct_name, decode_function_name, path_constant_name } }`, the concrete's per-kind settings, and `VectorKindConfiguration { kind, quantities: VectorKindQuantities { count, terms, message_length } }`, the family's per-kind configuration; no kind is compared with another and none with a declaration |

### Libraries

The constants library and then the records library, each over its `LibraryDocument`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| constants library | `generate.constants_library` | the file name's constructor over a clone of `file_name`, then the library name's over a clone of `library_name` | `GeneratedFileName::try_new(GeneratedFileNameConstructorParams { text })`, then `SolidityLibraryName::try_new(SolidityLibraryNameConstructorParams { text })` | `EvmLibraryNames { file_name, library_name }`; a file-name refusal `Err(AdmitGenerateErrorReturn::FileName { field: GenerateField::ConstantsLibraryFileName, error })`, a library-name refusal `Err(AdmitGenerateErrorReturn::LibraryName { field: GenerateField::ConstantsLibraryName, error })` |
| records library | `generate.records_library` | likewise | likewise | `EvmLibraryNames { file_name, library_name }`; a file-name refusal `Err(AdmitGenerateErrorReturn::FileName { field: GenerateField::RecordsLibraryFileName, error })`, a library-name refusal `Err(AdmitGenerateErrorReturn::LibraryName { field: GenerateField::RecordsLibraryName, error })` |

### Settings

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| Foundry project root | `generate.foundry_root` | none | none | a clone of the `PathBuf`, carried as it is |
| generated directory | each `(index, segment)` of `generate.generated_directory` in order | each segment's constructor over a clone of the segment, then the directory's | `GeneratedFileName::try_new(GeneratedFileNameConstructorParams { text })` per segment, then `GeneratedDirectory::try_new(GeneratedDirectoryConstructorParams { segments })` | the `GeneratedDirectory`; a segment refusal `Err(AdmitGenerateErrorReturn::FileName { field: GenerateField::GeneratedDirectorySegment { index }, error })`, a directory refusal `Err(AdmitGenerateErrorReturn::GeneratedDirectory(error))` |
| curve directories | `generate.curve_directories` | the bn254 text's constructor over a clone, then the bls12_381 text's | `GeneratedFileName::try_new(GeneratedFileNameConstructorParams { text })` twice | `EvmCurveDirectories { bn254, bls12_381 }`; a refusal `Err(AdmitGenerateErrorReturn::FileName { field: GenerateField::Bn254Directory, error })` or `Err(AdmitGenerateErrorReturn::FileName { field: GenerateField::Bls12381Directory, error })` |
| compiler version | `generate.compiler_version` | none | none | `SolidityCompilerVersion { major, minor, patch }`, each field the document's `compiler_version` field of its name |
| license | `generate.license` | the constructor over a clone of the text | `SpdxLicenseIdentifier::try_new(SpdxLicenseIdentifierConstructorParams { text })` | the `SpdxLicenseIdentifier`; a refusal `Err(AdmitGenerateErrorReturn::License(error))` |

### Computed

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| admitted | the concrete and every field admitted | none | none | `Ok(AdmitGenerateSuccessReturn { concrete: GenerateConcrete::Evm { settings: Box::new(EvmGenerateSettings { kinds: settings_kinds, constants_library, records_library, foundry_root, generated_directory, curve_directories, compiler_version, license }) }, kinds })`, `settings_kinds` the admitted `EvmVectorKindSettings` and `kinds` the admitted `VectorKindConfiguration`s, each in document order |

### Ordering

The concrete, then each kind in document order with its fields in the order above, then the constants library, the records library, the Foundry project root, the generated directory's segments and then the directory, the curve directories, the compiler version, then the license; the earliest refusal in that order returns; no side effect.
