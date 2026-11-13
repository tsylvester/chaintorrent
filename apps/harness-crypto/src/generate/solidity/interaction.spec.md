# `solidity` — interaction spec

Branch contract for the `solidity` module of the `generate` module of `apps/harness-crypto`: `solidity` is the Solidity concrete of the harness's generate family — one adapter over the borrowed pairing, hash-to-scalar adapter, encoder, randomness source, credential KEM, key agreement, and delivery proof with its chain forms. It calls each function it owns with each kind's admitted size, encodes every vector record into one `0x`-prefixed hex line, renders the constants library and the vector-record library through `render`, writes every vector file under the cell directory the received pairing and encoder declarations name beneath the configured generated directory, and writes both libraries under the configured generated directory outside every cell directory. Each function's branches state condition, decision, dependency call, and the exact return outcome.

## `GeneratedFileName::try_new`

`try_new(params: GeneratedFileNameConstructorParams) -> GeneratedFileNameTryNewReturn`, decided over `params.text.as_bytes()`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| empty | `params.text` has no byte | none | none | `Err(GeneratedFileNameTryNewErrorReturn::Empty)` |
| invalid | the lowest index `i` whose byte is outside `b'A'..=b'Z'`, `b'a'..=b'z'`, `b'0'..=b'9'`, `b'.'`, `b'_'`, and `b'-'` | that byte | none | `Err(GeneratedFileNameTryNewErrorReturn::InvalidCharacter { index: i, byte })` |
| reserved | the text is `.` or `..` | the text | none | `Err(GeneratedFileNameTryNewErrorReturn::Reserved)` |
| admitted | every byte is in the set and the text is neither `.` nor `..` | none | none | `Ok(GeneratedFileName { text })`, the text moved from the params |

## `GeneratedFileName::as_str`

`as_str(&self) -> &str`: one branch; outcome the held text.

## `GeneratedDirectory::try_new`

`try_new(params: GeneratedDirectoryConstructorParams) -> GeneratedDirectoryTryNewReturn`:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| empty | `params.segments` is empty | the length | none | `Err(GeneratedDirectoryTryNewErrorReturn::Empty)` |
| admitted | `params.segments` is not empty | none | none | `Ok(GeneratedDirectory { segments })`, the vector moved from the params in its order |

## `SolidityGenerate::try_new`

`try_new(params: SolidityGenerateConstructorParams) -> SolidityGenerateTryNewReturn`: one branch; outcome `Ok` of the concrete holding every field of the params.

## `IGenerateAdapter for SolidityGenerate<'_, P, E, K, A, D>`

`const DECLARATION: GenerateDeclaration` naming `SOLIDITY_GENERATE_DECLARATION`, which holds, in order, `G1Addition` `Drawn` `false`, `G1Multiplication` `Drawn` `false`, `PairingCheck` `Drawn` `false`, `G2Addition` `Drawn` `true`, `G1MultiScalarMultiplication` `DrawnTerms` `true`, `G2MultiScalarMultiplication` `DrawnTerms` `true`, `DecodeRejection` `Enumerated` `false`, `HashToScalar` `DrawnMessages` `false`, `IdentityMapping` `OverConfiguredInputs` `false`, `Possession` `Drawn` `false`, `Challenge` `Drawn` `false`, `MintProof` `Drawn` `false`, `TransferProof` `Drawn` `false`, and `CellValues` `Enumerated` `false`, each a `VectorKindDeclaration` of `kind`, `sizing`, and `requires_second_group_arithmetic`, with `adapter_version` `1`, `interface_version` `GENERATE_INTERFACE_VERSION`, and `encoding_identifier` `EncodingIdentifier::EthereumAbiV1`.

`type Forms = D::Forms`; `emit(&self, params: EmitParams, payload: EmitPayload<'_, D::Forms>) -> EmitReturn`, the trusted form. Every error below is returned as `Err(EmitErrorReturn::Solidity(error))`; `F` below is `D::Forms`, which the return does not name. The payload's contexts and identities arrive typed and admitted and are handed on unchanged; every function the concrete owns is called through the handle `self` holds for it.

### Emitted kinds

Decision an exhaustive `match` on `P::DECLARATION.verifier_group_arithmetic`; the emitted kinds are the declared kinds in declared order, a kind whose `requires_second_group_arithmetic` is `true` kept only under `BothGroups`.

### Settings of an emitted kind

Each emitted kind's names are the first entry of `self.settings.kinds` whose `kind` is the kind, and its size the first entry of `self.sizes` whose `kind` is the kind:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| names missing | the names are absent for an emitted kind | the first such kind in declared order | none | `Err(SolidityGenerateErrorReturn::MissingKindSettings { kind })` |
| size missing | the size is absent for an emitted kind | the first such kind in declared order | none | `Err(EmitErrorReturn::MissingKindSize { kind })` |
| admitted | every emitted kind has its names and its size | none | none | proceed |

Every emitted kind's names and size are read before any function is called; the concrete compares no size with its declaration, the factory having built each size from it.

### Constants

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| refusal | `constants` refuses | none | `(self.constants)(&ConstantsDeps { pairing: self.pairing, encoder: self.encoder }, ConstantsParams { statement_version: self.statement_version }, ConstantsPayload { kem: &K::DECLARATION, key_agreement: &A::DECLARATION, delivery_proof: &D::DECLARATION })` | `Err(SolidityGenerateErrorReturn::Constants(error))` |
| entries refusal | `SolidityLibraryEntries::try_new` refuses the returned entries | none | `SolidityLibraryEntries::try_new(SolidityLibraryEntriesConstructorParams { entries })` | `Err(SolidityGenerateErrorReturn::Entries(error))`; the constants function's names are unique, so no input takes this refusal |

### Group vectors

Dependency call `(self.group_vectors)(&GroupVectorsDeps { pairing: self.pairing, random: self.random }, GroupVectorsParams { first_group: FirstGroupVectorCounts { g1_add, g1_mul, pairing_check }, second_group }, GroupVectorsPayload)`, each first-group count the `Drawn` count of its kind, and `second_group` `Some(SecondGroupVectorCounts { g2_add, g1_msm: MsmVectorSettings { count, terms }, g2_msm: MsmVectorSettings { count, terms } })` from the second-group kinds' admitted sizes under `BothGroups` and `None` under `FirstGroupOnly`; a refusal returns `Err(SolidityGenerateErrorReturn::GroupVectors(error))`.

### Rejection vectors

Dependency call `(self.rejection_vectors)(&RejectionVectorsDeps { pairing: self.pairing }, RejectionVectorsParams, RejectionVectorsPayload)`, a refusal returning `Err(SolidityGenerateErrorReturn::RejectionVectors(error))`.

### Mapping vectors

Dependency call `(self.mapping_vectors)(&MappingVectorsDeps { pairing: self.pairing, hash_to_scalar: self.hash_to_scalar, kem: self.kem, random: self.random }, MappingVectorsParams { hash_to_scalar_count, message_length }, MappingVectorsPayload { key_agreement: &A::DECLARATION, delivery_proof: &D::DECLARATION, identities: payload.identities })`, the count and the message length the hash-to-scalar kind's `DrawnMessages`; a refusal returns `Err(SolidityGenerateErrorReturn::MappingVectors(error))`.

### Possession vectors

Dependency call `(self.possession_vectors)(&PossessionVectorsDeps { pairing: self.pairing, key_agreement: self.key_agreement, random: self.random }, PossessionVectorsParams { count }, PossessionVectorsPayload)`, the count the possession kind's `Drawn` count; a refusal returns `Err(SolidityGenerateErrorReturn::PossessionVectors(error))`.

### Challenge vectors

Dependency call `(self.challenge_vectors)(&ChallengeVectorsDeps { pairing: self.pairing, encoder: self.encoder, hash_to_scalar: self.hash_to_scalar, random: self.random }, ChallengeVectorsParams { count, statement_version: self.statement_version }, ChallengeVectorsPayload { delivery_proof: &D::DECLARATION, mint: payload.mint, transfer: payload.transfer })`, the count the challenge kind's `Drawn` count; a refusal returns `Err(SolidityGenerateErrorReturn::ChallengeVectors(error))`.

### Proof vectors

Dependency call `(self.proof_vectors)(&ProofVectorsDeps { pairing: self.pairing, kem: self.kem, key_agreement: self.key_agreement, delivery_proof: self.delivery_proof, random: self.random }, ProofVectorsParams { mint_count, transfer_count }, ProofVectorsPayload { mint: payload.mint, transfer: payload.transfer, identity: payload.proof_identity })`, the counts the mint-proof and transfer-proof kinds' `Drawn` counts; a refusal returns `Err(SolidityGenerateErrorReturn::ProofVectors(error))`, a delivery's error carried whole inside it.

### Lines, per emitted kind

Each record, in record order: dependency call `self.encoder.encode(EncodeParams { description: &description }, &record)` under the kind's description constructed by its `try_new` and unpacked irrefutably, a refusal returning `Err(SolidityGenerateErrorReturn::Encode(error))`, no input taking it. The line is `0x`, each encoded byte as two lowercase hex digits, and `\n`; the kind's contents are the lines concatenated.

### Cell directory

`GeneratedFileName::try_new(GeneratedFileNameConstructorParams { text })` over `P::DECLARATION.concrete_name`, the cell directory's first segment, and over `E::DECLARATION.name`, its second, each refusal returning `Err(SolidityGenerateErrorReturn::GeneratedFileName(error))`; the cell directory the two admitted names build is read once and used for every emitted kind's path and the directory the files are written under.

### Paths, per emitted kind

The path constant's text is the kind's configured `file_name` alone; `SolidityStringLiteral::try_new(SolidityStringLiteralConstructorParams { text })` over it, a refusal returning `Err(SolidityGenerateErrorReturn::Path(error))`, every file-name byte being printable ASCII the literal admits, so no input takes it. The filesystem path is `self.settings.foundry_root` joined with `self.settings.generated_directory`'s segments, the cell directory's two segments, and the file name.

### Declarations, per emitted kind in declared order

The members are built first, then `SolidityRecordDeclaration { struct_name, members, decode_function_name, path_constant_name, path }`, the names cloned from the kind's `names` and `members` the built `SolidityRecordMembers`. The members are built in this order, the first refusal returning:

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| names refusal | `member_names()` refuses | none | `member_names()` on the kind's description | `Err` of the description's variant — `GroupMembers`, `RejectionMembers`, `MappingMembers`, `PossessionMembers`, `ChallengeMembers`, `ProofMembers`, or `CellValuesMembers` — unchanged; no description takes it |
| count | the names' length differs from the length of the description's `FIELDS` | the two lengths, decided before any type name is read | none | `Err(SolidityGenerateErrorReturn::MemberCount { kind, expected, actual })`, `expected` the length of `FIELDS` and `actual` the names' length; no description takes it |
| type names | each index `kind` the `FIELDS` entry at that index | none | `self.encoder.type_name(TypeNameParams, kind)` unpacked irrefutably, its error arm uninhabited, then `SolidityAbiType::try_new(SolidityAbiTypeConstructorParams { name: success.name.as_str().to_owned() })` | `Err(SolidityGenerateErrorReturn::AbiType(error))` unchanged on refusal, which no name the ABI concrete declares takes; the member is `SolidityRecordMember { name, kind, abi_type }` with the name at that index |
| members refusal | `SolidityRecordMembers::try_new` refuses | none | `SolidityRecordMembers::try_new(SolidityRecordMembersConstructorParams { members })` | `Err(SolidityGenerateErrorReturn::RecordMembers(error))` unchanged, which no description's names take |

The type name is read from the borrowed encoder and never from a table in the concrete. Then `SolidityRecordDeclarations::try_new(SolidityRecordDeclarationsConstructorParams { declarations })`, a refusal returning `Err(SolidityGenerateErrorReturn::Declarations(error))`, the inputs that take it being a struct name, a decode function name, or a path constant name configured for two emitted kinds.

### Libraries

Dependency calls `(self.render)(&RenderDeps, RenderParams { license: self.settings.license.clone(), compiler_version: self.settings.compiler_version }, RenderPayload { library_name: self.settings.constants_library.library_name.clone(), body: SolidityLibraryBody::Constants(entries) })` and the same with `self.settings.records_library`'s library name and `SolidityLibraryBody::Records(declarations)`, each unpacked irrefutably.

### Writes

Dependency call `std::fs::create_dir_all` over the cell directory's filesystem path, a failure returning `Err(SolidityGenerateErrorReturn::CreateDirectory { path, error })`; then `std::fs::write` of each emitted kind's contents at its filesystem path in declared order, then the constants library's text at the generated directory joined with `self.settings.constants_library.file_name`, then the records library's text at the generated directory joined with `self.settings.records_library.file_name`, a failure returning `Err(SolidityGenerateErrorReturn::WriteFile { path, error })`. Each error is carried unchanged with its path; the inputs that take them are a configured root that is an existing file and a directory standing at a kind's file path.

### Computed

Outcome `Ok(EmitSuccessReturn { files })`, one `EmittedFile` per file written in write order, a kind's vector file carrying `kind` `Some` of that kind and `path` the path written, each library's carrying `kind` `None` and its path.

### Ordering

The emitted kinds and their settings, the constants, the group, rejection, mapping, possession, challenge, and proof vectors, the lines, the cell directory's admission, the paths, and the declarations, the libraries, then the directory and the writes. The first refusal returns, and a refusal before the writes leaves the filesystem untouched; `params` carries nothing and is not read.

## Integration: own entries

Each entry is proven by the private integration block of the same name in `integration_test.rs`, except `each_record_member_carries_the_encoders_type_name_for_its_kind`, proven in `test.rs` over the encoding family's mock encoder.

- `emit_writes_each_emitted_kind_then_both_libraries_in_write_order_under_the_cell_directory`: condition `emit` over a generated directory of two distinct segments and file names distinct across the kinds and the libraries; outcome `Ok` whose `files` are, in order, the root joined with the segments, the cell directory's two segments, and the file name of each emitted kind in declared order, then the constants library's configured file name, then the records library's, each existing, both libraries directly under the generated directory outside every cell directory; the cell directory's segments are the names the case's pairing's and encoder's declarations supply, and the kinds needing second-group arithmetic are emitted only where the case's pairing declares both groups; variation every member of `PAIRING_CONCRETES` crossed with every selection of `DECLARED_ENCODING_SELECTIONS`, so a swapped directory, a name read from configuration, an order other than the declared one, a gate that decides by anything else, or a segment order that differs fails; edge the paths are the real functions' kinds and the real filesystem's.
- `each_kinds_file_holds_one_hex_line_per_record_at_its_admitted_size`: condition `emit` with every kind's admitted count distinct from every other's, the term count and message length distinct from the counts, and two distinct identities; outcome each drawn kind's file holds its admitted count of lines except the challenge kind's, which holds a mint line and a transfer line per count, the hash-to-scalar file holds the admitted count for each of the identity, possession first-group, possession second-group, challenge, and weight domain tags, the identity-mapping file holds one line per configured identity, and the decode-rejection file holds one line for each record the case's pairing's declaration enumerates for the kind, and the cell-values file holds the one record `constants` returns; every line is `0x` and an even run of lowercase hex digits, each file ends in a line feed and holds no empty line; variation every member of `PAIRING_CONCRETES`; edge the counts are the sizes the settings carry, read from `sizes` and never from the configured quantities.
- `every_written_line_decodes_under_its_kinds_description`: condition every line of every written kind's file, its hex after `0x` read back into bytes, decoded through the case's encoder under the description of the kind that wrote it; outcome every line decodes and the decoded records are as many as the file's lines; variation every kind written, whose descriptions differ in fields and kinds, over every member of `DECLARED_ENCODING_SELECTIONS`, so a line encoded under another kind's description fails to decode; edge the bytes are those the encoder returned, relayed through the hex line.
- `the_constants_library_opens_with_the_configured_license_version_and_name_and_holds_the_tag_entries`: condition `emit` with a license, a compiler version, and a constants library name each other than its builder default; outcome the constants library, under its configured file name in the generated directory outside every cell directory, begins with the configured license line, the pragma of the configured compiler version, a blank line, and `library` followed by the configured constants library name and ` {`, and holds the line `    bytes internal constant CHALLENGE_TAG = hex"` followed by the lowercase hex of the delivery proof declaration's `challenge_tag` and `";`; variation every member of `PAIRING_CONCRETES`; edge the tag is the declaration's, carried through `constants` and `render`.
- `the_records_library_declares_each_written_kinds_struct_decode_function_and_path`: condition `emit` with a records library name other than its builder default, each kind's names distinct, and a generated directory of two distinct segments; outcome the records library begins with the configured header and the configured records library name, and for each written kind holds its struct, whose members carry the type names the Ethereum ABI encoder gives their kinds, `FixedBytes32` `bytes32`, `Unsigned16` `uint16`, `Unsigned32` `uint32`, `Unsigned64` `uint64`, `Text` `string`, `Bytes` `bytes`, `FixedBytes20` `bytes20`, and `Unsigned256` `uint256`, a `FixedBytes20` member typed `bytes20` and never `address`, its decode function with the kind's struct and name, and its path constant holding the kind's file name alone, and holds no struct, decode function, or path constant of a kind not written; variation every member of `PAIRING_CONCRETES`, the second-group kinds' declarations present only where the case's pairing declares second-group arithmetic; edge the path constant is the file name a reader joins to the cell directory it reads, and the expected type names are assembled in the test from the ABI table and never read from the subject.
- `each_record_member_carries_the_encoders_type_name_for_its_kind`: condition `emit` over the encoding family's mock encoder, whose `type_name` returns for each canonical field kind its declared name `mockFixedBytes32`, `mockUnsigned16`, `mockUnsigned32`, `mockUnsigned64`, `mockText`, `mockBytes`, `mockFixedBytes20`, or `mockUnsigned256`; outcome each emitted kind's declaration, as `render` receives it, holds its members in order, each carrying the name its description's `member_names()` declares at that index, the kind its description's `FIELDS` holds at that index, and the `SolidityAbiType` whose text is the mock's declared name for that kind; variation the kinds whose descriptions hold different canonical field kinds, so a member typed by another kind's name fails; edge the type names are read from the borrowed encoder and never from a table in the concrete.
- `a_constants_refusal_reaches_the_caller_unchanged_and_writes_nothing`: condition `emit` with a statement version other than `DELIVERY_STATEMENT_VERSION_ONE`; outcome `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::Constants(ConstantsErrorReturn::UnsupportedStatementVersion { version })))`, `version` the configured value, and the root does not exist; variation every member of `PAIRING_CONCRETES`; edge the refusal is `constants`'s own, carried whole.
- `a_mapping_vectors_refusal_reaches_the_caller_unchanged_and_writes_nothing`: condition `emit` over the hash-to-scalar adapter `create_hash_to_scalar` returns for `HashToScalarConcrete::Mock(MockIHashToScalarAdapterFailureMode::SamplingRefused)`; outcome `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::MappingVectors(MappingVectorsErrorReturn::HashToScalar(HashToScalarErrorReturn::Sampling(SampleUniformScalarErrorReturn::WrongLength { expected, actual })))))`, `expected` being `<P::Scalar as ISampleUniformScalar>::UNIFORM_BYTES_LENGTH` and `actual` the length the hash-to-scalar family declares for its `SamplingRefused` mode, and the root does not exist; variation every member of `PAIRING_CONCRETES`, the refusal arriving after the constants, group, and rejection functions have run; edge nothing is written though earlier functions succeeded.
- `a_repeated_struct_name_is_refused_with_its_indexes_and_writes_nothing`: condition `emit` with the first two emitted kinds' settings carrying the same struct name and distinct file names; outcome `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::Declarations(SolidityRecordDeclarationsTryNewErrorReturn::DuplicateStructName { first_index: 0, duplicate_index: 1 })))` and the root does not exist; variation every member of `PAIRING_CONCRETES`; edge the refusal is `render`'s constructor's, carried whole after every function has run.
- `a_repeated_decode_function_name_is_refused_with_its_indexes_and_writes_nothing`: condition as the struct name entry with the same decode function name; outcome `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::Declarations(SolidityRecordDeclarationsTryNewErrorReturn::DuplicateDecodeFunctionName { first_index: 0, duplicate_index: 1 })))` and the root does not exist; variation every member of `PAIRING_CONCRETES`; edge the refusal is `render`'s constructor's, carried whole.
- `a_repeated_path_constant_name_is_refused_with_its_indexes_and_writes_nothing`: condition as the struct name entry with the same path constant name; outcome `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::Declarations(SolidityRecordDeclarationsTryNewErrorReturn::DuplicatePathConstantName { first_index: 0, duplicate_index: 1 })))` and the root does not exist; variation every member of `PAIRING_CONCRETES`; edge the refusal is `render`'s constructor's, carried whole.
- `a_repeated_struct_name_is_refused_before_a_repeated_decode_function_name`: condition as the struct name entry with the same struct name and the same decode function name; outcome `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::Declarations(SolidityRecordDeclarationsTryNewErrorReturn::DuplicateStructName { first_index: 0, duplicate_index: 1 })))`, the struct name's refusal taking precedence, and the root does not exist; variation every member of `PAIRING_CONCRETES`; edge the constructor's order of checks survives composition.
- `a_root_that_is_a_file_refuses_directory_creation_with_its_path`: condition `emit` with a regular file standing at the configured root; outcome `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::CreateDirectory { path, error }))`, `path` the root joined with the generated directory's segments and the cell directory's two segments, and `error` equal in `kind()`, `raw_os_error()`, and `to_string()` to the error `std::fs::create_dir_all` returns for that path in the same state; variation every member of `PAIRING_CONCRETES`; edge the error is the filesystem's, carried unchanged with the path it concerned.
- `a_directory_at_a_kinds_file_path_refuses_the_write_with_its_path`: condition `emit` with a directory standing at the first emitted kind's file path; outcome `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::WriteFile { path, error }))`, `path` that file path and `error` equal in `kind()`, `raw_os_error()`, and `to_string()` to the error `std::fs::write` returns for it in the same state, and the constants library's file does not exist; variation every member of `PAIRING_CONCRETES`; edge the error is the filesystem's, carried unchanged, and no file after the refused one is written.

## Integration: callee dispositions

- `hash-to-scalar/factory` entry `every_concrete_returns_the_sampling_bound_refusal_whole`: absorbed, proven by `a_mapping_vectors_refusal_reaches_the_caller_unchanged_and_writes_nothing` in the private integration test element `integration_test.rs`, restated in this node's terms as: the refusal reaches the caller of `emit` whole as `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::MappingVectors(MappingVectorsErrorReturn::HashToScalar(error))))`.
- `encoding/factory` entry `every concrete decodes what it encodes`: absorbed, proven by `every_written_line_decodes_under_its_kinds_description` in the private integration test element `integration_test.rs`, restated in this node's terms as: the bytes `encode` returns for a record under its kind's description, written as its line, decode through `decode` under the same description.
- `pairing/factory` entry `a_sampling_refusal_reaches_the_caller_unchanged`, carried through `group_vectors` and `challenge_vectors`: carried, restated in this node's terms as: the refusal reaches the caller of `emit` whole as `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::GroupVectors(GroupVectorsErrorReturn::SampleScalar(error))))` or `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::ChallengeVectors(ChallengeVectorsErrorReturn::SampleScalar(error))))`; route `create_pairing`, the arm's concrete's `sample_from_uniform_bytes`, `group_vectors` or `challenge_vectors`, then `emit`; the outer-edge collaborator on the route is the pairing family's mock concrete selected by `PairingConcrete::Mock(MockIPairingAdapterFailureMode::SampleScalarWrongLength)`.
- `pairing/factory` entry `a_sampled_scalar_encodes_through_secret`, carried through `group_vectors`: carried, restated in this node's terms as: every sampled scalar's encoding held in a record's `draws` is encoded into that record's line; route `create_pairing`, the arm's concrete's `sample_from_uniform_bytes` and `encode_scalar`, `group_vectors`, then the encoder's `encode` and the write in `emit`; the outer-edge collaborator on the route is the random source whose draw is the sampler's input.
- `pairing/factory` entries `libraries_sharing_an_identifier_return_the_same_scalar_field_order`, `libraries_sharing_an_identifier_return_the_same_first_group_reference_encoding`, and `libraries_sharing_an_identifier_return_the_same_second_group_reference_encoding`, carried through `rejection_vectors`: carried, restated in this node's terms as: the decode-rejection kind's file holds, per record in order, the line of the record's encoding, so its lines are equal across any two concretes whose declared target-group encoding identifiers are equal; route `create_pairing`, the arm's concrete through its `try_new`, `rejection_vectors`, then the encoder's `encode` and the write in `emit`; the route reaches no outer-edge collaborator other than the filesystem.
- `kem/factory` entry `every_concrete_returns_a_hash_to_scalar_refusal_whole`, carried through `mapping_vectors`: carried, restated in this node's terms as: the refusal reaches the caller of `emit` whole as `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::MappingVectors(MappingVectorsErrorReturn::Setup(SetupErrorReturn::HashToScalar(error)))))` under the asset scope and as `Err(EmitErrorReturn::Solidity(SolidityGenerateErrorReturn::MappingVectors(MappingVectorsErrorReturn::DeriveIdentity(DeriveIdentityErrorReturn::HashToScalar(error)))))` under the entitlement scope; route `create_pairing`, the arm's concrete through its `try_new`, `create_hash_to_scalar` selecting the mock concrete under `MockIHashToScalarAdapterFailureMode::SamplingRefused`, `create_kem`, the selected KEM concrete's `try_new`, `setup` or `derive_identity` inside `mapping_vectors`, then `emit`; the outer-edge collaborator on the route is the hash-to-scalar concrete, the family's mock concrete under its failure mode.
- `proof_vectors` carried entry `every_concrete_declaring_the_keccak256_identifier_hashes_the_context_the_encoding_factory_encodes_to_the_oracle`: carried, restated where this node transforms it: each mint-proof and transfer-proof record's `identity_element` is written unchanged into its kind's line through the encoder's `encode` under its description, so the line decodes to the encoding the entry states for the admitted proof identity; route `create_encoding`, `create_pairing`, the arm's concrete through its `try_new`, `create_hash_to_scalar`, the selected concrete's `try_new`, `create_kem`, the selected concrete's `try_new`, `mint`, `derive_identity`, `transfer`, `identity_element_components`, `proof_vectors`, `encode_g1`, then the encoder's `encode` and the write in `emit`; the outer-edge collaborator on the route is the filesystem.

## Integration: private surface

The chain of real functions `create_pairing`, the arm's concrete through its `try_new`, `create_hash_to_scalar`, the selected concrete's `try_new` and `hash_to_scalar`, `create_encoding`, the selected encoder's `encode` and `decode`, `create_chain_forms`, `create_kem`, `create_key_agreement`, `create_delivery_proof`, each selected concrete through its `try_new`, `create_random_source` for `RandomSourceKind::OperatingSystem`, `build_solidity_generate`'s `SolidityGenerate::try_new`, `emit`, `constants`, `group_vectors`, `rejection_vectors`, `mapping_vectors`, `possession_vectors`, `challenge_vectors`, `proof_vectors`, `mint`, `transfer`, `render`, each description's `try_new`, `to_fields`, `fields_to_value`, and `member_names`, the selected encoder's `type_name`, `SolidityLibraryEntries::try_new`, `SolidityAbiType::try_new`, `SolidityRecordMembers::try_new`, `SolidityRecordDeclarations::try_new`, and `SolidityStringLiteral::try_new`; the cases are every member of `PAIRING_CONCRETES` crossed with the selections `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declare `HashToScalarIdentifier::Keccak256V1`, `DECLARED_ENCODING_SELECTIONS`, `CHAIN_FORMS_CONCRETES`, `DECLARED_KEM_SELECTIONS`, `DECLARED_KEY_AGREEMENT_SELECTIONS`, and `DELIVERY_PROOF_CONCRETES` hold; the outer-edge collaborators are the filesystem, a directory beneath the operating system's temporary directory removed before each act, the operating system's generator running real, and the hash-to-scalar family's mock concrete under `SamplingRefused` for the mapping refusal entry alone; the observable result is the `EmitReturn`, the files read back, and the encoder's decode results; the entries proven are every own entry above but `each_record_member_carries_the_encoders_type_name_for_its_kind`, which `test.rs` proves over the encoding family's mock encoder and a production-typed `RenderFn` declared in the block, and the absorbed entries; every block hands `build_solidity_generate` the real function of each module the concrete owns as that module's handle, so the chain the entries prove is real.
