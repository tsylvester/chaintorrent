# evm interactions

`EvmGenerate` is the generate family's EVM concrete: `emit` writes each admitted vector kind's file — one `0x`-prefixed lowercase-hex line per record — then the constants library and the records library, each beneath the configured Foundry root, the generated directory's segments, and the curve directory the pairing's declaration selects. The concrete computes no constant and no vector — each comes from the function modules it owns — renders no text itself, which `render` does, and encodes every record through the injected encoder; it touches the filesystem only at the writes. It names no curve, count, tag, name, or path: the adapters arrive by injection, the scope and statement version in its params, the settings and the admitted sizes the factory builds in its constructor.

## `GeneratedFileName::try_new`

`try_new(params: GeneratedFileNameConstructorParams) -> GeneratedFileNameTryNewReturn`, decided over `params.text.as_bytes()`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| empty | no byte | the length check | none | `Err(GeneratedFileNameTryNewErrorReturn::Empty)` |
| invalid | the lowest index `i` whose byte is outside `b'A'..=b'Z'`, `b'a'..=b'z'`, `b'0'..=b'9'`, `b'.'`, `b'_'`, and `b'-'` | the per-byte class check | none | `Err(GeneratedFileNameTryNewErrorReturn::InvalidCharacter { index: i, byte })` |
| reserved | the text is `.` or `..` | equality | none | `Err(GeneratedFileNameTryNewErrorReturn::Reserved)` |
| admitted | none of the above | none | none | `Ok(GeneratedFileName { text })` |

`as_str(&self) -> &str` returns the text.

## `GeneratedDirectory::try_new`

`try_new(params: GeneratedDirectoryConstructorParams) -> GeneratedDirectoryTryNewReturn`, a directory relative to the Foundry project root:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| empty | `params.segments` is empty | the length check | none | `Err(GeneratedDirectoryTryNewErrorReturn::Empty)` |
| admitted | `params.segments` is not empty | none | none | `Ok(GeneratedDirectory { segments })` |

## `EvmGenerate::try_new`

`try_new(params: EvmGenerateConstructorParams<'_, P, E, K, A, D>) -> EvmGenerateTryNewReturn<'_, P, E, K, A, D>`: one branch, outcome `Ok` of the concrete holding every field of the params.

## `IGenerateAdapter for EvmGenerate<'_, P, E, K, A, D>`

`const DECLARATION: GenerateDeclaration` naming `EVM_GENERATE_DECLARATION`, which holds, in order, each a `VectorKindDeclaration` of `kind`, `sizing`, and `requires_second_group_arithmetic`:

| Kind | Sizing | Second group |
|---|---|---|
| `G1Addition` | `Drawn` | `false` |
| `G1Multiplication` | `Drawn` | `false` |
| `PairingCheck` | `Drawn` | `false` |
| `G2Addition` | `Drawn` | `true` |
| `G1MultiScalarMultiplication` | `DrawnTerms` | `true` |
| `G2MultiScalarMultiplication` | `DrawnTerms` | `true` |
| `DecodeRejection` | `Enumerated` | `false` |
| `HashToScalar` | `DrawnMessages` | `false` |
| `IdentityMapping` | `OverConfiguredInputs` | `false` |
| `Possession` | `Drawn` | `false` |
| `Challenge` | `Drawn` | `false` |
| `MintProof` | `Drawn` | `false` |
| `TransferProof` | `Drawn` | `false` |

with `adapter_version` `1` and `interface_version` `GENERATE_INTERFACE_VERSION`.

## `emit`

`emit(&self, params: EmitParams, payload: EmitPayload<'_>) -> EmitReturn`, the trusted form; every error below but the identities' is returned as `Err(EmitErrorReturn::Evm(error))`; `F` below is `D::Forms`, which the return does not name.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| asset identity wrong length | under `IdentityScope::Asset`, the bytes of an identity at index `i` do not convert into `[u8; ASSET_IDENTITY_HASH_LENGTH]` | the conversion | none | `Err(EmitErrorReturn::AssetIdentityLength { index: i, actual })`, `actual` the bytes' length |
| asset identity refused | under `IdentityScope::Asset`, `AssetIdentityHash::try_new` refuses the converted bytes at index `i` | the try result | `AssetIdentityHash::try_new`, once per asset identity | `Err(EmitErrorReturn::AssetIdentityHash { index: i, error })`, the error unchanged |
| proof identity wrong length | under `IdentityScope::Asset`, `payload.proof_identity` does not convert into `[u8; ASSET_IDENTITY_HASH_LENGTH]` | the conversion | none | `Err(EmitErrorReturn::AssetProofIdentityLength { actual })`, `actual` the bytes' length |
| proof identity refused | under `IdentityScope::Asset`, `AssetIdentityHash::try_new` refuses the converted proof identity | the try result | `AssetIdentityHash::try_new`, once | `Err(EmitErrorReturn::AssetProofIdentityHash(error))`, the error unchanged |
| missing settings | an emitted kind has no settings entry or no size entry | the per-kind lookups | none | `Err(EvmGenerateErrorReturn::MissingKindSettings { kind })` |
| constants refused | `constants` returns `Err(error)` | the function's return | `constants::<P, F>` | `Err(EvmGenerateErrorReturn::Constants(error))`, unchanged |
| entries refused | `SolidityLibraryEntries::try_new` returns `Err(error)` | the constructor's refusal | `SolidityLibraryEntries::try_new` | `Err(EvmGenerateErrorReturn::Entries(error))`, unchanged |
| group vectors refused | `group_vectors` returns `Err(error)` | the function's return | `group_vectors::<P>` | `Err(EvmGenerateErrorReturn::GroupVectors(error))`, unchanged |
| rejection refused | `rejection_vectors` returns `Err(error)` | the function's return | `rejection_vectors::<P>` | `Err(EvmGenerateErrorReturn::RejectionVectors(error))`, unchanged |
| mapping refused | `mapping_vectors` returns `Err(error)` | the function's return | `mapping_vectors::<P, K>` | `Err(EvmGenerateErrorReturn::MappingVectors(error))`, unchanged |
| possession refused | `possession_vectors` returns `Err(error)` | the function's return | `possession_vectors::<P, A>` | `Err(EvmGenerateErrorReturn::PossessionVectors(error))`, unchanged |
| challenge refused | `challenge_vectors` returns `Err(error)` | the function's return | `challenge_vectors::<P, E, F>` | `Err(EvmGenerateErrorReturn::ChallengeVectors(error))`, unchanged |
| proof refused | `proof_vectors` returns `Err(error)` | the function's return | `proof_vectors::<P, K, A, D>` | `Err(EvmGenerateErrorReturn::ProofVectors(error))`, unchanged |
| encode refused | `self.encoder.encode` returns `Err(error)` | the encode result | `self.encoder.encode`, once per record | `Err(EvmGenerateErrorReturn::Encode(error))`, unchanged |
| members refused | a kind's description's `members()` returns `Err(error)` | the members result | the description's `members` | `Err` of the kind's members variant — `GroupMembers`, `RejectionMembers`, `MappingMembers`, `PossessionMembers`, `ChallengeMembers`, or `ProofMembers` — carrying `error` unchanged |
| path refused | `SolidityStringLiteral::try_new` refuses the kind's relative path | the constructor's refusal | `SolidityStringLiteral::try_new`, once per emitted kind | `Err(EvmGenerateErrorReturn::Path(error))`, unchanged |
| declarations refused | `SolidityRecordDeclarations::try_new` returns `Err(error)` | the constructor's refusal | `SolidityRecordDeclarations::try_new` | `Err(EvmGenerateErrorReturn::Declarations(error))`, unchanged |
| create refused | `std::fs::create_dir_all` fails | the create result | `std::fs::create_dir_all`, once | `Err(EvmGenerateErrorReturn::CreateDirectory { path, error })`, the path and the error unchanged |
| write refused | `std::fs::write` fails | the write result | `std::fs::write`, once per file | `Err(EvmGenerateErrorReturn::WriteFile { path, error })`, the path and the error unchanged |
| computed | every file written | `files` in write order | the calls stated | `Ok(EmitSuccessReturn { files })`, one `EmittedFile` per file written in write order, a kind's vector file carrying `kind` `Some` of that kind and `path` the path written, each library's carrying `kind` `None` and its path |

### Identities

Decision an exhaustive `match` on `self.scope`: under `IdentityScope::Entitlement` each identity and the proof identity is the canonical identity, admitted as given; under `IdentityScope::Asset`, each index `i` and bytes of `payload.identities` in order takes the conversion, a failure taking the asset-identity-wrong-length branch, then dependency call `AssetIdentityHash::try_new(AssetIdentityHashConstructorParams { bytes })`, a refusal taking the asset-identity-refused branch; then `payload.proof_identity` takes the same conversion, a failure taking the proof-identity-wrong-length branch, and the same dependency call, a refusal taking the proof-identity-refused branch. Each refusal is the family's flavor, which every concrete, the family's mock concrete included, decides from its scope and the payload's identities before its vendor runs, so it precedes every setting read and every function called.

### Emitted kinds

Decision an exhaustive `match` on `P::DECLARATION.verifier_group_arithmetic`: the declared kinds in declared order, a kind whose `requires_second_group_arithmetic` is `true` kept only under `BothGroups`.

### Settings of an emitted kind

Its names, from the first entry of `self.settings.kinds` whose `kind` is the kind, and its size, from the first entry of `self.sizes` whose `kind` is the kind; either absent takes the missing-settings branch. Every emitted kind's names and size are read before any function is called; the concrete compares no size with its declaration, the factory having built each size from it.

### Constants

Dependency call `constants::<P, F>(&ConstantsDeps { pairing: self.pairing }, ConstantsParams { statement_version: self.statement_version }, ConstantsPayload { kem: &K::DECLARATION, key_agreement: &A::DECLARATION, delivery_proof: &D::DECLARATION })`, a refusal taking the constants-refused branch; then `SolidityLibraryEntries::try_new(SolidityLibraryEntriesConstructorParams { entries })`, a refusal taking the entries-refused branch — the constants function's names are unique, so no input takes that refusal.

### Group vectors

Dependency call `group_vectors::<P>(&GroupVectorsDeps { pairing: self.pairing, random: self.random }, GroupVectorsParams { first_group: FirstGroupVectorCounts { g1_add, g1_mul, pairing_check }, second_group }, GroupVectorsPayload)`, each first-group count the `Drawn` count of its kind, and `second_group` `Some(SecondGroupVectorCounts { g2_add, g1_msm: MsmVectorSettings { count, terms }, g2_msm: MsmVectorSettings { count, terms } })` from the second-group kinds' admitted sizes under `BothGroups` and `None` under `FirstGroupOnly`; a refusal takes the group-vectors-refused branch.

### Rejection vectors

Dependency call `rejection_vectors::<P>(&RejectionVectorsDeps { pairing: self.pairing }, RejectionVectorsParams, RejectionVectorsPayload)`, a refusal taking the rejection-refused branch.

### Mapping vectors

Dependency call `mapping_vectors::<P, K>(&MappingVectorsDeps { pairing: self.pairing, hash_to_scalar: self.hash_to_scalar, kem: self.kem, random: self.random }, MappingVectorsParams { hash_to_scalar_count, message_length, scope: self.scope }, MappingVectorsPayload { key_agreement: &A::DECLARATION, delivery_proof: &D::DECLARATION, identities: payload.identities })`, the count and length the hash-to-scalar kind's `DrawnMessages`; a refusal takes the mapping-refused branch — its `AssetIdentityLength` and `AssetIdentityHash` arms are reached by no input, the identities having been admitted above under the scope it receives.

### Possession vectors

Dependency call `possession_vectors::<P, A>(&PossessionVectorsDeps { pairing: self.pairing, key_agreement: self.key_agreement, random: self.random }, PossessionVectorsParams { count }, PossessionVectorsPayload)`; a refusal takes the possession-refused branch.

### Challenge vectors

Dependency call `challenge_vectors::<P, E, F>(&ChallengeVectorsDeps { pairing: self.pairing, encoder: self.encoder, hash_to_scalar: self.hash_to_scalar, random: self.random }, ChallengeVectorsParams { count, statement_version: self.statement_version }, ChallengeVectorsPayload { delivery_proof: &D::DECLARATION, mint: payload.mint, mint_purpose: payload.mint_purpose, transfer: payload.transfer, transfer_purpose: payload.transfer_purpose })`; a refusal takes the challenge-refused branch.

### Proof vectors

Dependency call `proof_vectors::<P, K, A, D>(&ProofVectorsDeps { pairing: self.pairing, kem: self.kem, key_agreement: self.key_agreement, delivery_proof: self.delivery_proof, random: self.random }, ProofVectorsParams { mint_count, transfer_count, scope: self.scope }, ProofVectorsPayload { mint: payload.mint, mint_purpose: payload.mint_purpose, transfer: payload.transfer, transfer_purpose: payload.transfer_purpose, identity: payload.proof_identity })`, the counts the mint-proof and transfer-proof kinds' `Drawn` counts; a refusal takes the proof-refused branch — its `AssetIdentityLength` and `AssetIdentityHash` arms are reached by no input, the proof identity having been admitted above under the scope it receives.

### Lines

Per emitted kind: each record, in record order, dependency call `self.encoder.encode(EncodeParams { description: &description }, &record)` under the kind's description constructed by its `try_new` and unpacked irrefutably, a refusal taking the encode-refused branch — no input takes it; the line `0x`, each encoded byte as two lowercase hex digits, and `\n`; the kind's contents the lines concatenated.

### Paths

Per emitted kind: the relative path `self.settings.generated_directory`'s segments, the curve directory, and the kind's `file_name`, joined by `/`, the curve directory `self.settings.curve_directories.bn254` or `.bls12_381` by an exhaustive `match` on `P::DECLARATION.curve`; `SolidityStringLiteral::try_new(SolidityStringLiteralConstructorParams { text })` over it, a refusal taking the path-refused branch — every file-name byte being printable ASCII the literal admits, so no input takes it; the filesystem path `self.settings.foundry_root` joined with each segment, the curve directory, and the file name.

### Declarations

Per emitted kind in declared order: `SolidityRecordDeclaration { struct_name, members, decode_function_name, path_constant_name, path }`, the names cloned from the kind's `names`, `members` the kind's description's `members()`, a refusal taking the kind's members variant — which no description takes; then `SolidityRecordDeclarations::try_new(SolidityRecordDeclarationsConstructorParams { declarations })`, a refusal taking the declarations-refused branch — the inputs that take it being a struct name, a decode function name, or a path constant name configured for two emitted kinds.

### Libraries

Dependency calls `render(&RenderDeps, RenderParams { license: self.settings.license.clone(), compiler_version: self.settings.compiler_version }, RenderPayload { library_name: self.settings.constants_library.library_name.clone(), body: SolidityLibraryBody::Constants(entries) })` and the same with the records library's name and `SolidityLibraryBody::Records(declarations)`, each unpacked irrefutably.

### Writes

Dependency call `std::fs::create_dir_all` over the curve directory's filesystem path, a failure taking the create-refused branch; then `std::fs::write` of each emitted kind's contents at its filesystem path in declared order, then the constants library's text at the curve directory joined with `constants_library.file_name`, then the records library's text at the curve directory joined with `records_library.file_name`, a failure taking the write-refused branch; each error carried unchanged with its path — the inputs that take them being a configured root that is an existing file and a directory standing at a kind's file path.

### Ordering

The identities, then the emitted kinds and their settings, the constants, the group, rejection, mapping, possession, challenge, and proof vectors, the lines, paths, and declarations, the libraries, then the directory and the writes; the first refusal returns, and a refusal before the writes leaves the filesystem untouched; `params` carries nothing and is not read.

## Integration: callee dispositions

- `every_concrete_returns_the_sampling_bound_refusal_whole` (`hash-to-scalar/factory`): absorbed — proven by `a_mapping_vectors_refusal_reaches_the_caller_unchanged_and_writes_nothing` in the private integration test element `integration_test.rs`; restated in this node's terms: the refusal reaches the caller of `emit` whole as `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::MappingVectors(MappingVectorsErrorReturn::HashToScalar(error))))`.
- `every concrete decodes what it encodes` (`encoding/factory`): absorbed — proven by `every_written_line_decodes_under_its_kinds_description` in the private integration test element `integration_test.rs`; restated in this node's terms: the bytes `encode` returns for a record under its kind's description, written as its line, decode through `decode` under the same description.
- `a_sampling_refusal_reaches_the_caller_unchanged` (`pairing/factory`), carried through `group_vectors` and `challenge_vectors`: carried — restated in this node's terms: the refusal reaches the caller of `emit` whole as `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::GroupVectors(GroupVectorsErrorReturn::SampleScalar(error))))` or `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::ChallengeVectors(ChallengeVectorsErrorReturn::SampleScalar(error))))`. Route: `create_pairing`, the arm's concrete's `sample_from_uniform_bytes`, `group_vectors` or `challenge_vectors`, then `emit`; the outer-edge collaborator on the route is the pairing family's mock concrete selected by `PairingConcrete::Mock(MockIPairingAdapterFailureMode::SampleScalarWrongLength)`.
- `a_sampled_scalar_encodes_through_secret` (`pairing/factory`), carried through `group_vectors`: carried — restated in this node's terms: every sampled scalar's encoding held in a record's `draws` is encoded into that record's line. Route: `create_pairing`, the arm's concrete's `sample_from_uniform_bytes` and `encode_scalar`, `group_vectors`, then the encoder's `encode` and the write in `emit`; the outer-edge collaborator on the route is the random source whose draw is the sampler's input.
- `libraries_sharing_an_identifier_return_the_same_scalar_field_order`, `libraries_sharing_an_identifier_return_the_same_first_group_reference_encoding`, and `libraries_sharing_an_identifier_return_the_same_second_group_reference_encoding` (`pairing/factory`), carried through `rejection_vectors`: carried — restated in this node's terms: the decode-rejection kind's file holds, per record in order, the line of the record's encoding, so its lines are equal across any two concretes whose declared target-group encoding identifiers are equal. Route: `create_pairing`, the arm's concrete through its `try_new`, `rejection_vectors`, then the encoder's `encode` and the write in `emit`; the route reaches no outer-edge collaborator other than the filesystem.
- `every_concrete_returns_a_hash_to_scalar_refusal_whole` (`kem/factory`), carried through `mapping_vectors`: carried — restated in this node's terms: the refusal reaches the caller of `emit` whole as `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::MappingVectors(MappingVectorsErrorReturn::Setup(SetupErrorReturn::HashToScalar(error)))))` under the asset scope and as `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::MappingVectors(MappingVectorsErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::HashToScalar(error)))))` under the entitlement scope. Route: `create_pairing`, the arm's concrete through its `try_new`, `create_hash_to_scalar` selecting the mock concrete under `MockIHashToScalarAdapterFailureMode::SamplingRefused`, `create_kem`, the selected KEM concrete's `try_new`, `setup` or `derive_identity` inside `mapping_vectors`, then `emit`; the outer-edge collaborator on the route is the hash-to-scalar concrete, the family's mock concrete under its failure mode.

## Integration: own entries

Each entry is named and proven by the private integration block of the same name in `integration_test.rs`.

- `emit_writes_each_emitted_kind_then_both_libraries_in_write_order_under_the_curve_directory`: condition `emit` over a generated directory of two distinct segments, a distinct directory text per curve, and file names distinct across the kinds and the libraries; outcome `Ok` whose `files` are, in order, the root joined with the segments, the curve directory, and the file name of each emitted kind in declared order, then the constants library's file name, then the records library's, each existing; the curve directory is the configured text of the curve the case's pairing is on, and the kinds needing second-group arithmetic are emitted only where the case's pairing is on the curve whose verifier has both groups; variation every member of `PAIRING_CONCRETES`, both curves, keyed on the case's curve and never on the declaration the function reads, so a swapped directory, an order other than the declared one, a gate that decides by anything else, or a segment order that differs fails; edge that must survive the composition: the paths are the real functions' kinds and the real filesystem's.
- `each_kinds_file_holds_one_hex_line_per_record_at_its_admitted_size`: condition `emit` with every kind's admitted count distinct from every other's, the term count and message length distinct from the counts, and two distinct identities; outcome each drawn kind's file holds its admitted count of lines except the challenge kind's, which holds a mint line and a transfer line per count, the hash-to-scalar file holds the admitted count for each of the identity, possession first-group, possession second-group, challenge, and weight domain tags, the identity-mapping file holds one line per configured identity, and the decode-rejection file holds one line for each candidate the case's curve defines, the outside-the-subgroup second-group point and the scalar field order on the curve whose first group is the whole curve and the first-group point as well on the other; every line is `0x` and an even run of lowercase hex digits, each file ends in a line feed and holds no empty line; variation every member of `PAIRING_CONCRETES`; edge that must survive the composition: the counts are the sizes the settings carry, read from `sizes` and never from the configured quantities.
- `every_written_line_decodes_under_its_kinds_description`: condition every line of every written kind's file, its hex after `0x` read back into bytes, decoded through the case's encoder under the description of the kind that wrote it; outcome every line decodes and the decoded records are as many as the file's lines; variation every kind written, whose descriptions differ in fields and kinds, over every member of `DECLARED_ENCODING_SELECTIONS`, so a line encoded under another kind's description fails to decode; edge that must survive the composition: the bytes are those the encoder returned, relayed through the hex line.
- `the_constants_library_opens_with_the_configured_license_version_and_name_and_holds_the_tag_entries`: condition `emit` with a license, a compiler version, and a constants library name each other than its builder default; outcome `Constants.sol`, under the configured file name, begins with the configured license line, the pragma of the configured compiler version, a blank line, and `library` followed by the configured name and ` {`, and holds the line `    bytes internal constant CHALLENGE_TAG = hex"` followed by the lowercase hex of the delivery proof declaration's `challenge_tag` and `";`; variation every member of `PAIRING_CONCRETES`; edge that must survive the composition: the tag is the declaration's, carried through `constants` and `render`.
- `the_records_library_declares_each_written_kinds_struct_decode_function_and_path`: condition `emit` with a records library name other than its builder default, each kind's names distinct, and a generated directory of two distinct segments; outcome the records library begins with the configured header and library name, and for each written kind holds its struct, its decode function with the kind's struct and name, and its path constant holding the segments, the curve directory, and the kind's file name joined by `/`, and holds no struct, decode function, or path constant of a kind not written; variation every member of `PAIRING_CONCRETES`, the second-group kinds' declarations present only on the curve that writes them; edge that must survive the composition: the path in the library is the path the file was written at, relative to the root.
- `a_constants_refusal_reaches_the_caller_unchanged_and_writes_nothing`: condition `emit` with a statement version other than `DELIVERY_STATEMENT_VERSION_ONE`; outcome `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::Constants(ConstantsErrorReturn::UnsupportedStatementVersion { version })))`, `version` the configured value, and the root does not exist; variation every member of `PAIRING_CONCRETES`; edge that must survive the composition: the refusal is `constants`'s own, carried whole.
- `a_mapping_vectors_refusal_reaches_the_caller_unchanged_and_writes_nothing`: condition `emit` over the hash-to-scalar adapter `create_hash_to_scalar` returns for `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)`; outcome `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::MappingVectors(MappingVectorsErrorReturn::HashToScalar(HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength { expected, actual })))))`, `expected` being `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` and `actual` the length the hash-to-scalar family declares for its `SamplingRefused` mode, and the root does not exist; variation every member of `PAIRING_CONCRETES`, the refusal arriving after the constants, group, and rejection functions have run; edge that must survive the composition: nothing is written though earlier functions succeeded.
- `a_repeated_struct_name_is_refused_with_its_indexes_and_writes_nothing`: condition `emit` with the first two emitted kinds' settings carrying the same struct name and distinct file names; outcome `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::Declarations(SolidityRecordDeclarationsTryNewErrorReturn::DuplicateStructName { first_index: 0, duplicate_index: 1 })))` and the root does not exist; variation every member of `PAIRING_CONCRETES`; edge that must survive the composition: the refusal is `render`'s constructor's, carried whole after every function has run.
- `a_repeated_decode_function_name_is_refused_with_its_indexes_and_writes_nothing`: condition as the struct name entry with the same decode function name; outcome `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::Declarations(SolidityRecordDeclarationsTryNewErrorReturn::DuplicateDecodeFunctionName { first_index: 0, duplicate_index: 1 })))` and the root does not exist; variation every member of `PAIRING_CONCRETES`; edge that must survive the composition: the refusal is `render`'s constructor's, carried whole.
- `a_repeated_path_constant_name_is_refused_with_its_indexes_and_writes_nothing`: condition as the struct name entry with the same path constant name; outcome `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::Declarations(SolidityRecordDeclarationsTryNewErrorReturn::DuplicatePathConstantName { first_index: 0, duplicate_index: 1 })))` and the root does not exist; variation every member of `PAIRING_CONCRETES`; edge that must survive the composition: the refusal is `render`'s constructor's, carried whole.
- `a_repeated_struct_name_is_refused_before_a_repeated_decode_function_name`: condition as the struct name entry with the same struct name and the same decode function name; outcome `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::Declarations(SolidityRecordDeclarationsTryNewErrorReturn::DuplicateStructName { first_index: 0, duplicate_index: 1 })))`, the struct name's refusal taking precedence, and the root does not exist; variation every member of `PAIRING_CONCRETES`; edge that must survive the composition: the constructor's order of checks survives composition.
- `a_root_that_is_a_file_refuses_directory_creation_with_its_path`: condition `emit` with a regular file standing at the configured root; outcome `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::CreateDirectory { path, error }))`, `path` the root joined with the generated directory's segments and the curve directory, and `error` equal in `kind()`, `raw_os_error()`, and `to_string()` to the error `std::fs::create_dir_all` returns for that path in the same state; variation every member of `PAIRING_CONCRETES`; edge that must survive the composition: the error is the filesystem's, carried unchanged with the path it concerned.
- `a_directory_at_a_kinds_file_path_refuses_the_write_with_its_path`: condition `emit` with a directory standing at the first emitted kind's file path; outcome `Err(EmitErrorReturn::Evm(EvmGenerateErrorReturn::WriteFile { path, error }))`, `path` that file path and `error` equal in `kind()`, `raw_os_error()`, and `to_string()` to the error `std::fs::write` returns for it in the same state, and the constants library's file does not exist; variation every member of `PAIRING_CONCRETES`; edge that must survive the composition: the error is the filesystem's, carried unchanged, and no file after the refused one is written.

## Private surface

The chain of real functions: `create_pairing`, the arm's concrete through its `try_new`, `create_hash_to_scalar`, the selected concrete's `try_new` and `hash_to_scalar`, `create_encoding`, the selected encoder's `encode` and `decode`, `create_chain_forms`, `create_kem`, `create_key_agreement`, `create_delivery_proof`, each selected concrete through its `try_new`, `create_random_source` for `RandomSourceKind::OperatingSystem`, `build_evm_generate`'s `EvmGenerate::try_new`, `emit`, `constants`, `group_vectors`, `rejection_vectors`, `mapping_vectors`, `possession_vectors`, `challenge_vectors`, `proof_vectors`, `render`, each description's `try_new`, `to_fields`, `fields_to_value`, and `members`, `SolidityLibraryEntries::try_new`, `SolidityRecordDeclarations::try_new`, and `SolidityStringLiteral::try_new`; the cases are every member of `PAIRING_CONCRETES` crossed with the selections `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declare `HashToScalarIdentifier::Keccak256V1`, `DECLARED_ENCODING_SELECTIONS`, `CHAIN_FORMS_CONCRETES`, `DECLARED_KEM_SELECTIONS`, `DECLARED_KEY_AGREEMENT_SELECTIONS`, and `DELIVERY_PROOF_CONCRETES` hold; the outer-edge collaborators are the filesystem, a directory beneath the operating system's temporary directory removed before each act, the operating system's generator running real, and the hash-to-scalar family's mock concrete under `SamplingRefused` for the mapping refusal entry alone; the observable result is the `EmitReturn`, the files read back, and the encoder's decode results; the entries proven are every own entry above and the absorbed entries.
