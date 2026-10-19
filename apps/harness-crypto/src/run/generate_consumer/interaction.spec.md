# `generate_consumer` — interaction spec

Branch contract for the `generate_consumer` module of the `run` module of `apps/harness-crypto`: `GenerateConsumer`, an `IGenerateConsumer` the run constructs over a borrow of the configured `HarnessStatements` and hands to `create_generate` as the deps' consumer, which writes the generated directory by calling `emit` once on whichever concrete it is handed, with the configured mint and transfer contexts, purposes, identities, and proof identity, and relays `emit`'s return unchanged. Each branch states condition, decision, dependency call, and the exact return outcome.

## `GenerateConsumer::try_new(params: GenerateConsumerConstructorParams<'a>) -> GenerateConsumerTryNewReturn<'a>`

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| constructed | always | none | none | `Ok(GenerateConsumer { statements: params.statements })` |

The error arm is uninhabited (`Result<_, Infallible>`): a borrow of admitted statements is a whole consumer, so no branch produces an error.

## `consume_generate<G: IGenerateAdapter>(&self, _params: ConsumeGenerateParams, payload: ConsumeGeneratePayload<G>) -> EmitReturn`

`impl IGenerateConsumer for GenerateConsumer<'_>` with `type Output = EmitReturn;`. The trusted form; the adapter arrives from `create_generate`, generic over the family's trait — the consumer names no concrete, no concrete's error, and no concrete's settings.

| Branch | Condition | Decision | Dependency call | Outcome |
|---|---|---|---|---|
| emitted | `emit` returns `Ok(success)` | none | `payload.adapter.emit(EmitParams, EmitPayload { mint: &self.statements.mint, mint_purpose: self.statements.mint_purpose, transfer: &self.statements.transfer, transfer_purpose: self.statements.transfer_purpose, identities: &self.statements.identities, proof_identity: &self.statements.proof_identity })`, once | `Ok(success)`, unchanged |
| refused | `emit` returns `Err(error)` | none | the same call | `Err(error)`, unchanged |

## Ordering and invariants

- `emit` is called once; `params` carries nothing and is not read; there is no side effect of the consumer's own, every write being the concrete's.
- The consumer calls no factory, reads no clock, touches no filesystem itself, and holds no literal; every value it hands `emit` is the statements' own.

## Integration: own entries

Each entry is proven by the public integration block of the same name in `tests/generate_consumer_integration_test.rs`.

- `the_configured_identities_reach_the_identity_mapping_file`: condition `create_generate`, over the selection `build_generate_concrete` returns for a member of `GENERATE_CONCRETES` and the kinds `build_generate_kind_configurations` returns for it, hands its adapter to the consumer constructed over statements whose `identities` hold three distinct identities; outcome `Ok` whose one `EmittedFile` of `kind` `Some(VectorKind::IdentityMapping)` names a file holding exactly three lines; variation every member of `GENERATE_CONCRETES` crossed with every member of `PAIRING_CONCRETES` and the other families' declared sets, the identities' count differing from the statements builder's default and from the proof identity's, so a consumer that hands `emit` the builder's identities, the proof identity in their place, or a shortened list fails; edge the identity-mapping kind holds one record per configured identity, the file found by its emitted kind tag and read from the real filesystem.

## Integration: callee dispositions

The `harness-crypto/generate/factory` entries this node's route reaches.

- `a_refused_write_returns_the_mock_concretes_own_error`: absorbed — proven by the private integration block of the same name in `integration_test.rs`; restated in this node's terms as: under `MockIGenerateAdapterFailureMode::WriteRefused` the consumer's output is `Err(EmitErrorReturn::Mock(MockIGenerateAdapterEmitErrorReturn::WriteRefused))`, the error whole.
- `a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds`: absorbed — proven by the private integration block of the same name in `integration_test.rs`; restated in this node's terms as: the possession kind's configured count reaches, through the consumer's `emit`, the possession file as exactly that count of lines.
- `emit_returns_a_distinct_path_for_every_kind_the_declaration_does_not_gate`: carried; route `create_pairing`, the arm's concrete through its `try_new`, `create_hash_to_scalar`, the selected concrete's `try_new` and `hash_to_scalar`, `create_encoding`, `create_chain_forms`, `create_kem`, `create_key_agreement`, `create_delivery_proof`, each selected concrete through its `try_new`, `create_random_source` for `RandomSourceKind::OperatingSystem`, `create_generate`, the selected concrete's `try_new`, `GenerateConsumer::consume_generate`, `emit`, and the functions `emit` runs, ending at this node; the outer-edge collaborators on that route are the filesystem, a directory beneath the operating system's temporary directory, and the operating system's generator; the consumer relays `emit`'s return unchanged, so the entry is not restated.

## Integration: private surface

The chain of real functions the carried entry's route names, with `GenerateConsumer::try_new` ahead of `create_generate`; the cases are every member of `PAIRING_CONCRETES` crossed with the selections `DECLARED_HASH_TO_SCALAR_SELECTIONS` that declare `HashToScalarIdentifier::Keccak256V1`, `DECLARED_ENCODING_SELECTIONS`, `CHAIN_FORMS_CONCRETES`, `DECLARED_KEM_SELECTIONS`, `DECLARED_KEY_AGREEMENT_SELECTIONS`, `DELIVERY_PROOF_CONCRETES`, and `GENERATE_CONCRETES`, the generate family's mock selection in place of `GENERATE_CONCRETES` for the refused-write entry; the outer-edge collaborators are the filesystem, a directory beneath the operating system's temporary directory, the operating system's generator running real, and the generate family's mock concrete under `WriteRefused` for the refused-write entry alone; the observable result is the consumer's `EmitReturn` and the files it names, read back; the entries proven are `a_refused_write_returns_the_mock_concretes_own_error` and `a_kinds_configured_count_reaches_the_concrete_as_the_records_its_file_holds`.

## Integration: public surface

`lib.rs` re-exports `run::provides`, which re-exports this module's surface whole, so the consumer is a public entry; an outside caller constructs `GenerateConsumer` by `GenerateConsumer::try_new` from `GenerateConsumerConstructorParams` over a borrow of `HarnessStatements`, passes it as `CreateGenerateDeps`' `consumer` to `create_generate` from `harness_crypto` with `CreateGenerateParams` selecting a member of `GENERATE_CONCRETES` and the adapters the other families' factories hand it, and observes the consumer's `EmitReturn` as `CreateGenerateSuccessReturn`'s `output` and the files it names; the route is the chain the private surface names, reached through the crate's public surface and the adapter crates' public surfaces; the outer-edge collaborators are the filesystem, a directory beneath the operating system's temporary directory, and the operating system's generator running real; the entry proven is `the_configured_identities_reach_the_identity_mapping_file`.
