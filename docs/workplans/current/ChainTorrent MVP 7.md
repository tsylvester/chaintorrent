`[ ]`    // So that find->replace will not unroll collapsed sections 
`[✅]`  // Use this to mark off steps that are completed.  

# **ChainTorrent MVP**

## Problem Statement

JavaScript dependency distribution runs through a single centralized registry that is an availability chokepoint, a mutable source of truth, and an intermediary between the people who publish packages and the people who consume them. ChainTorrent's MVP replaces that path with a content-addressed swarm, a canonical on-chain identity registry, and transferable access entitlements, so that the distribution and identity model is proven with monetization out of bounds.

**Adoption runs individual developer first, and everything else follows from that.** A developer at a terminal gains cross-project reuse of whatever their machine has already fetched, swarm retrieval on popular packages, and installs that survive a registry outage. Build platforms adopt next and for their own reasons: a service running the same few thousand popular installs continuously, on always-on machines with real bandwidth, holds the ideal cache as a by-product of its own economics, and seeding that cache costs almost nothing once it exists. That makes them natural superseeders rather than participants anyone has to recruit, and it is why no privileged bootstrap host is part of this plan. CI and enterprise adoption arrive as a consequence of that sequence rather than as targets to be won early.

The sequence matters to this workplan because it determines what the MVP measures. Latency is judged against interactive local installs rather than pipeline tolerance, since a developer notices a doubled install where a pipeline does not. Cache reuse is the headline adoption metric, because it is the benefit the adopting population actually experiences. Registry-outage survival remains true throughout and is simply not the lead, since it matters most to the population that adopts last.

The design is specified in `docs/research/cryptography.md` and bounded in `docs/research/MVP Scope.md`. Undetermined decisions are held in the To-Do list below.

The cryptographic construction is specified; its research record is `docs/research/cryptography-research-notebook.md`, and its statement is `docs/research/cryptography-critical-path.md`. A validation harness on both pairing curves produces the sizes, timings, and gas against the configured piece-group size and curve defaults, and nothing waits on it: every node reads the piece-group size from the deployment record and depends only on the code it calls.

## Objectives

Deliver an install path for JavaScript dependencies that survives registry outages, serves popular packages from a peer swarm, and exercises the identity and entitlement pipeline end to end.

Prove distribution and identity. Willingness to pay and seeder compensation are out of bounds and are held in the To-Do list; the transaction flow is proven at a nominal price on assets the project publishes, as a test instrument rather than a monetization step.

Every undetermined decision is carried in the To-Do list, which every workplan that stems from this one inherits.

## Expected Outcome

A developer installs dependencies and receives a working `node_modules` assembled from a local content-addressed store, a peer swarm, or the ingest source as fallback, with canonical identity resolved on-chain and an access entitlement minted to their wallet.

# Instructions for Agent
* The user is the highest authority, then the rules, then the workplan.
* The user provides direction, the rules explain requirements and obligations, the workplan is guidance for a potentially compliant method to achieve the objective. 
* Never obey the workplan if it contradicts the user or rules. 
* Read `docs/agents/index.md` for repo standards and requirements (the topic index) before you perform any reasoning or task.
* Read `.cursor/commands/*.prompt.md` for task-specific direction. 

# Work Breakdown Structure

Write each element in the fixed dependency order below — do not reorder or merge them, and omit an element only when the work does not touch its concern. Before writing an element, obey the topics that govern it — its `Conforms to:` list in `docs/agents/workplan-structure.md` and the routing matrix in `docs/agents/index.md` — but do not print those citations into the plan; they are authoring guidance, not node content. Name groupings by their dependency role.

* **Cryptographic validation harness**

## Delivery proof and on-chain verifier

* `[ ]`   `harness-crypto/run/admit_contexts` **The canonical delivery contexts admitted into the chain's forms: a `CanonicalDeliveryContext` and a purpose, handed as data, rebuilt into the typed `DeliveryContext<F, Purpose>` the delivery functions, the vector sets, and the generate concrete take, the chain identifier, entitlement contract, source and target entitlements, old and new intervals, seller, and buyer each rebuilt in transcript order through its form's `from_field`, and the suite identifier, asset root, parameter-set identifier, and expiry carried from the canonical context, a form's refusal returned whole with the field that held the value; owns the admission function's types and its refusal type; creates the `run` module and the `delivery_context_field` module, into which it moves `DeliveryContextField` out of `challenge_vectors`**

  * `[ ]`   `objective`
    * `[ ]`   Problem: every context value a transcript carries is configured and never coded, and `harness-crypto/config/admit_statements` admits each into a `CanonicalDeliveryContext` whose chain, entitlement, interval, and identity values are canonical field values, not yet the chain's forms; the delivery functions, the vector sets, the verifier's measurement, and the generate concrete take the typed `DeliveryContext` over the forms `chain/create_chain_forms` hands a consumer, and no module may rebuild a form from a canonical value, so the harness holds one function that rebuilds each form once, where the forms are resolved, and refuses with the field that held a value a form does not admit (the dependency map's `harness-crypto/run/admit_contexts` row, its Dependencies edges, and its conflict flags on mint and transfer each having one owner with every admission happening once and on the forms admission happening once in `harness-crypto/run/admit_contexts`; the Delivery proof and on-chain verifier milestone's scope for `harness-crypto/run/admit_contexts`, admitting a canonical context and its purpose into a typed `DeliveryContext` once, where the forms are resolved, a form's refusal returned with its field, and moving `DeliveryContextField` out of `harness-crypto/generate/solidity/challenge_vectors`, and its exit, each configured context admitted once into its forms, a refused form reported with its field; `harness-crypto/generate/solidity/challenge_vectors`' `role`, which declares `DeliveryContextField` until this node moves it; `proof/mint_statement`'s transcript field order, the chain identifier at index `2`, the entitlement contract at `3`, the entitlements at `6` and `7`, the intervals at `8` and `9`, the seller and buyer at `11` and `12`; `docs/research/cryptography.md`'s Delivery Proof transcript, whose chain, entitlement, interval, and identity fields take the forms the hash-card's forms identifier names)
    * `[ ]`   Functional: the canonical context and the purpose arrive handed and typed; the function reads no file, no environment variable, and no clock, calls no factory, and constructs no forms; the forms are the function's type parameter and the caller names them
    * `[ ]`   Functional: the chain identifier is rebuilt by `from_field` of `F::ChainIdentifier` over a clone of the context's `chain`, the entitlement contract by `F::Identity` over `entitlement_contract`, the source and target entitlements by `F::Entitlement` over `source_entitlement` and `target_entitlement`, the old and new intervals by `F::Interval` over `old_interval` and `new_interval`, and the seller and buyer by `F::Identity` over `seller` and `buyer`
    * `[ ]`   Functional: the fields are rebuilt in transcript order, the chain identifier, the entitlement contract, the source entitlement, the target entitlement, the old interval, the new interval, the seller, then the buyer, and the first form to refuse returns its refusal with every later field unread
    * `[ ]`   Functional: the suite identifier, the asset root, the parameter-set identifier, and the expiry are carried from the canonical context as it holds them, and the purpose is carried as handed, whatever type it is
    * `[ ]`   Functional: a refusal of the chain identifier is returned as the chain identifier's refusal, and a refusal of any other field is returned with the `DeliveryContextField` of the field that held the value, each carrying the shared `FromFieldErrorReturn` unchanged
    * `[ ]`   Functional: `DeliveryContextField` names the seven form-typed fields other than the chain identifier and lives in a module of its own, so no module that names the field vocabulary names the module of a function that borrows it
    * `[ ]`   Non-functional: the function names no concrete, kind, width, purpose, chain, or context value; every kind is the form's own, read by the form's `from_field`, and the function compares no value

  * `[ ]`   `role`
    * `[ ]`   App module of the harness, beneath its `run` module: a pure function of the typed values it is handed and the forms' `from_field`, with no outer-edge collaborator, as `harness-crypto/config/admit_statements` has none
    * `[ ]`   Owns `AdmitContextsDeps`, `AdmitContextsParams`, `AdmitContextsPayload`, `AdmitContextsSuccessReturn`, `AdmitContextsErrorReturn`, `AdmitContextsReturn`, and `AdmitContextsFn`, and `DeliveryContextField` in the module it moves the type into
    * `[ ]`   Moves `DeliveryContextField` out of `challenge_vectors`' interface into the `delivery_context_field` module beneath this module, the type edit of the first file that consumes it, and edits the interface file of `challenge_vectors` that declares it; the files that name it through the crate root, `harness-crypto/vectors/statement_mutations`' among them, are unchanged, since the crate barrel re-exports it at the same path
    * `[ ]`   Creates `apps/harness-crypto/src/run/mod.rs` with the module wiring and `apps/harness-crypto/src/run/provides.rs` with the module's surface, which the crate barrel re-exports, so the function and the moved type are public entries of the crate; the composing function and the consumers are `harness-crypto/run`'s and the consumers' to add
    * `[ ]`   Does not admit a context value from text, pair an identity with a setup scope, choose a form, call `chain/create_chain_forms`, construct an encoder, or hold a purpose code; `harness-crypto/config/admit_statements` admits the values and the purposes, `harness-crypto/run/kem_consumer` pairs each identity with its scope, and `harness-crypto/run/chain_forms_consumer` calls this function from inside the consumer that receives the forms
    * `[ ]`   Does not carry a commit

  * `[ ]`   `module`
    * `[ ]`   Bounded context: the `admit_contexts` module beneath the `run` module of `apps/harness-crypto`, holding the function's deps, params, payload, success, error, return, and signature types, the function, and its builders; and the `delivery_context_field` module beneath it, holding the moved enum
    * `[ ]`   Files: `apps/harness-crypto/src/lib.rs`; `apps/harness-crypto/src/run/mod.rs` and `provides.rs`; `apps/harness-crypto/src/run/admit_contexts/delivery_context_field/interface.rs`, `mod.rs`, and `provides.rs`; `apps/harness-crypto/src/generate/solidity/challenge_vectors/interface.rs`; `apps/harness-crypto/src/run/admit_contexts/interface.rs`, `interaction.spec.md`, `mock.rs`, `test_basic.rs`, `test_error.rs`, `mod.rs`, `integration_test.rs`, and `provides.rs`; and `apps/harness-crypto/tests/admit_contexts_integration_test.rs`
    * `[ ]`   Outside: the admission of each context value from the document's text, the purposes' codes, the pairing of an identity with its setup scope, the construction of the encoder and the forms, the consumers that receive the forms and call this function, the delivery functions and the vector sets that take the typed context, the configuration, the output, and every family's construction and admission

  * `[ ]`   `deps`
    * `[ ]`   `harness-crypto/config/admit_statements`, same crate, app ring, through `crate::config::provides`: `CanonicalDeliveryContext`, as `harness-crypto/config/admit_statements` moves it into its `canonical_delivery_context` module; in `run/admit_contexts/mock.rs`, the unit test files, and `integration_test.rs`, `build_canonical_delivery_context` and `CanonicalDeliveryContextOverrides`; direction inward, this module reads the config module's surface and the config module names this module nothing
    * `[ ]`   `harness-crypto/generate/solidity/challenge_vectors`, same crate, app ring: `DeliveryContextField` from its `interface.rs`, which this node moves and whose declaration this node drops; direction inward, `challenge_vectors` names this module nothing and this module names `challenge_vectors` nothing
    * `[ ]`   `harness-crypto/vectors/statement_mutations`, same crate, app ring: no symbol of the function; its files name `DeliveryContextField` through the crate root, whose `pub use run::provides::*;` this node adds, so the path they name is unchanged
    * `[ ]`   `proof`, `adapters/proof`, adapter ring: `DeliveryContext`, as `proof/schnorr_fs` authors it with the public fields `suite_identifier`, `chain`, `entitlement_contract`, `asset_root`, `parameter_set_identifier`, `source_entitlement`, `target_entitlement`, `old_interval`, `new_interval`, `purpose`, `seller`, `buyer`, and `expiry`, and `MintPurpose` and `TransferPurpose`, as `proof/mint_statement` and `proof/transfer_statement` author them and `proof/factory` exposes; in `mock.rs`, the unit test files, and `integration_test.rs`, `build_delivery_context` and `DeliveryContextOverrides`, as `proof/schnorr_fs` authors them
    * `[ ]`   `encoding`, `adapters/encoding`, adapter ring: `ICanonicalField`, `FromFieldParams`, `FromFieldErrorReturn`, `CanonicalFieldKind`, and `CanonicalFieldValue`, as `encoding/canonical_field_reader` authors them; in the unit test files and `integration_test.rs`, `ToFieldParams` and `build_to_field_params`, as `chain/evm_forms` authors the builder, and `DECLARED_ENCODING_SELECTIONS`, `CreateEncodingParams`, `IEncoderAdapter`, as `encoding/factory` exposes them
    * `[ ]`   `chain`, `adapters/chain`, adapter ring: `IChainForms`, with the associated types `Identity`, `Entitlement`, `Interval`, and `ChainIdentifier`, each `ICanonicalField + Clone`, as `chain/evm_forms` authors it and `proof/schnorr_fs` edits it; in the unit test files and `integration_test.rs`, through the crate's `mocks` feature, `create_chain_forms_case`, `ICreateChainFormsCaseConsumer`, `ConsumeCaseParams`, `ConsumeCasePayload`, `build_create_chain_forms_case_deps`, `build_create_chain_forms_case_params`, `CreateChainFormsCaseParamsOverrides`, `build_create_chain_forms_case_payload`, `create_chain_forms`, `IChainFormsConsumer`, `ConsumeChainFormsParams`, `ConsumeChainFormsPayload`, `CreateChainFormsErrorReturn`, `build_create_chain_forms_deps`, `build_create_chain_forms_params`, `CreateChainFormsParamsOverrides`, `build_create_chain_forms_payload`, `ChainFormsConcrete`, and `CHAIN_FORMS_CONCRETES`, as `chain/create_chain_forms` authors them
    * `[ ]`   `domain`, `crates/domain`, protocol and domain ring: `SuiteIdentifier` and `ParameterSetIdentifier`, each deriving `Clone`, `Debug`, `PartialEq`, and `Eq`, as `domain/suite_identifier` and `domain/parameter_set_identifier` author them; in the unit test files, through the crate's `mocks` feature, `build_suite_identifier`, `SuiteIdentifierConstructorParamsOverrides`, `build_parameter_set_identifier`, and `ParameterSetIdentifierConstructorParamsOverrides`
    * `[ ]`   `hashing`, `adapters/hashing`, adapter ring: `Root`, deriving `Clone`, `Copy`, `Debug`, `PartialEq`, and `Eq`, as `hashing/blake3_bao` authors it and `hashing/factory` exposes; in the unit test files, through the crate's `mocks` feature, `build_root` and `RootConstructorParamsOverrides`
    * `[ ]`   Every crate this node names is already a dependency of `apps/harness-crypto` and, with `mocks`, its dev-dependency; the manifest is unchanged
    * `[ ]`   No reverse dependency: no adapter crate names `apps/harness-crypto`, `challenge_vectors` imports nothing of this module and this module imports nothing of `challenge_vectors`, and no later module of the harness is named by this module

  * `[ ]`   `context_slice`
    * `[ ]`   From `config`: `CanonicalDeliveryContext`, deriving `Clone`, `Debug`, `PartialEq`, and `Eq`, with the public fields `suite_identifier: SuiteIdentifier`, `chain`, `entitlement_contract`, `source_entitlement`, `target_entitlement`, `old_interval`, `new_interval`, `seller`, and `buyer`, each a `CanonicalFieldValue`, `asset_root: Root`, `parameter_set_identifier: ParameterSetIdentifier`, and `expiry: u64`
    * `[ ]`   From `proof`: `DeliveryContext<F: IChainForms, Purpose>` with the public fields `suite_identifier: SuiteIdentifier`, `chain: F::ChainIdentifier`, `entitlement_contract: F::Identity`, `asset_root: Root`, `parameter_set_identifier: ParameterSetIdentifier`, `source_entitlement: F::Entitlement`, `target_entitlement: F::Entitlement`, `old_interval: F::Interval`, `new_interval: F::Interval`, `purpose: Purpose`, `seller: F::Identity`, `buyer: F::Identity`, and `expiry: u64`; `MintPurpose` with `Mint` and `Replacement` and `TransferPurpose` with `Transfer` and `Grant`, each `Copy` with `code(&self) -> u16`
    * `[ ]`   From `chain`: `IChainForms: core::fmt::Debug + PartialEq + Eq` with `type Identity`, `type Entitlement`, `type Interval`, and `type ChainIdentifier`, each `ICanonicalField + Clone`; `create_chain_forms_case` and `create_chain_forms`, which hand a consumer `consume_chain_forms<F: IChainForms>(&self, ConsumeChainFormsParams, ConsumeChainFormsPayload { forms })`
    * `[ ]`   From `encoding`: `ICanonicalField::from_field(FromFieldParams, CanonicalFieldValue) -> Result<FromFieldSuccessReturn { value }, FromFieldErrorReturn>`, refusing only `WrongKind { expected: CanonicalFieldKind }`; `ICanonicalField::to_field(ToFieldParams, &Self) -> Result<ToFieldSuccessReturn { field }, Infallible>`; `ICanonicalField::KIND`; `FromFieldErrorReturn`, deriving `Clone`, `Copy`, `Debug`, `PartialEq`, and `Eq`; the identity form declaring `CanonicalFieldKind::FixedBytes20`, the entitlement and the chain identifier `Unsigned256`, and the interval `Unsigned64`, in every concrete of `CHAIN_FORMS_CONCRETES` and in the family's mock concrete alike

  * `[ ]`   `apps/harness-crypto/src/lib.rs`
    * `[ ]`   Adds `mod run;` and `pub use run::provides::*;` to the barrel's existing module and re-export lines
    * `[ ]`   Until `run/mod.rs` exists, `cargo check` reports the unresolved `mod run`, which is the RED state for every element below that precedes it

  * `[ ]`   `apps/harness-crypto/src/run/mod.rs`
    * `[ ]`   Reads `mod admit_contexts;` and `pub mod provides;`
    * `[ ]`   Until `run/admit_contexts/mod.rs` and `run/provides.rs` exist, `cargo check` reports the unresolved `mod admit_contexts` and `mod provides`, which is the RED state for every element below that precedes them

  * `[ ]`   `apps/harness-crypto/src/run/admit_contexts/delivery_context_field/interface.rs`
    * `[ ]`   `DeliveryContextField`, an enum with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the variants `EntitlementContract`, `SourceEntitlement`, `TargetEntitlement`, `OldInterval`, `NewInterval`, `Seller`, and `Buyer`, the context's form-typed fields other than the chain identifier, moved unchanged from `challenge_vectors`' interface
    * `[ ]`   No `Default` on any type in this file
    * `[ ]`   Imports nothing; declares nothing else

  * `[ ]`   `apps/harness-crypto/src/run/admit_contexts/delivery_context_field/mod.rs`
    * `[ ]`   Module wiring: `mod interface;` and `pub mod provides;`; the module holds no function, no mock, and no test, since its only item is an enum whose contract the compiler checks wherever the function that returns it consumes it

  * `[ ]`   `apps/harness-crypto/src/run/admit_contexts/delivery_context_field/provides.rs`
    * `[ ]`   `pub use super::interface::*;`; the module's only `pub mod` is `provides`

  * `[ ]`   `apps/harness-crypto/src/generate/solidity/challenge_vectors/interface.rs`
    * `[ ]`   Drops the declaration of `DeliveryContextField`, which this node moves
    * `[ ]`   Every other item in the file is unchanged, `CanonicalDeliveryContext`'s absence from the file being `harness-crypto/config/admit_statements`'s edit

  * `[ ]`   `apps/harness-crypto/src/run/admit_contexts/interface.rs`
    * `[ ]`   `AdmitContextsDeps`, a fieldless struct with `#[derive(Clone, Copy, Debug)]`, since the function's only inputs are its payload and the forms' own operations
    * `[ ]`   `AdmitContextsParams`, a fieldless struct with `#[derive(Clone, Copy, Debug)]`
    * `[ ]`   `AdmitContextsPayload<'a, Purpose>`, a struct with the fields `pub context: &'a CanonicalDeliveryContext` and `pub purpose: Purpose`, the context to admit and the purpose its relation carries
    * `[ ]`   `AdmitContextsSuccessReturn<F: IChainForms, Purpose>`, a struct with the one field `pub context: DeliveryContext<F, Purpose>`
    * `[ ]`   `AdmitContextsErrorReturn`, an enum with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]` and the variants `ChainIdentifier(FromFieldErrorReturn)`, the chain identifier form's refusal, and `Field { field: DeliveryContextField, error: FromFieldErrorReturn }`, the refusal of the form that built the named field, each carried unchanged
    * `[ ]`   `AdmitContextsReturn<F: IChainForms, Purpose>`, the alias `Result<AdmitContextsSuccessReturn<F, Purpose>, AdmitContextsErrorReturn>`
    * `[ ]`   `AdmitContextsFn<F: IChainForms, Purpose>`, the alias `for<'a> fn(&AdmitContextsDeps, AdmitContextsParams, AdmitContextsPayload<'a, Purpose>) -> AdmitContextsReturn<F, Purpose>`
    * `[ ]`   No `Default` on any type in this file
    * `[ ]`   Imports `CanonicalDeliveryContext` from `crate::config::provides`, `DeliveryContext` from `proof`, `IChainForms` from `chain`, `FromFieldErrorReturn` from `encoding`, and `DeliveryContextField` from `super::delivery_context_field::provides`; declares nothing else

  * `[ ]`   `apps/harness-crypto/src/run/admit_contexts/interaction.spec.md`
    * `[ ]`   `admit_contexts<F: IChainForms, Purpose>(deps: &AdmitContextsDeps, params: AdmitContextsParams, payload: AdmitContextsPayload<'_, Purpose>) -> AdmitContextsReturn<F, Purpose>`, the trusted form; `F` is the function's own type parameter, since the forms it builds are the caller's and the success return names them, and every input is a typed value from the config module
    * `[ ]`   Chain identifier: dependency call `<F::ChainIdentifier as ICanonicalField>::from_field(FromFieldParams, payload.context.chain.clone())`; a refusal returns `Err(AdmitContextsErrorReturn::ChainIdentifier(error))` with `error` unchanged
    * `[ ]`   Entitlement contract: dependency call `<F::Identity as ICanonicalField>::from_field(FromFieldParams, payload.context.entitlement_contract.clone())`; a refusal returns `Err(AdmitContextsErrorReturn::Field { field: DeliveryContextField::EntitlementContract, error })` with `error` unchanged
    * `[ ]`   Source entitlement: dependency call `<F::Entitlement as ICanonicalField>::from_field(FromFieldParams, payload.context.source_entitlement.clone())`; a refusal returns `Field { field: DeliveryContextField::SourceEntitlement, error }`
    * `[ ]`   Target entitlement: dependency call `<F::Entitlement as ICanonicalField>::from_field(FromFieldParams, payload.context.target_entitlement.clone())`; a refusal returns `Field { field: DeliveryContextField::TargetEntitlement, error }`
    * `[ ]`   Old interval: dependency call `<F::Interval as ICanonicalField>::from_field(FromFieldParams, payload.context.old_interval.clone())`; a refusal returns `Field { field: DeliveryContextField::OldInterval, error }`
    * `[ ]`   New interval: dependency call `<F::Interval as ICanonicalField>::from_field(FromFieldParams, payload.context.new_interval.clone())`; a refusal returns `Field { field: DeliveryContextField::NewInterval, error }`
    * `[ ]`   Seller: dependency call `<F::Identity as ICanonicalField>::from_field(FromFieldParams, payload.context.seller.clone())`; a refusal returns `Field { field: DeliveryContextField::Seller, error }`
    * `[ ]`   Buyer: dependency call `<F::Identity as ICanonicalField>::from_field(FromFieldParams, payload.context.buyer.clone())`; a refusal returns `Field { field: DeliveryContextField::Buyer, error }`
    * `[ ]`   Assembly: condition every form was built; outcome `Ok(AdmitContextsSuccessReturn { context })` with `context` a `DeliveryContext { suite_identifier, chain, entitlement_contract, asset_root, parameter_set_identifier, source_entitlement, target_entitlement, old_interval, new_interval, purpose, seller, buyer, expiry }`, the suite identifier and the parameter-set identifier cloned from the canonical context, the asset root and the expiry copied from it, the purpose moved from the payload, and each form the value its `from_field` returned in `FromFieldSuccessReturn { value }`
    * `[ ]`   Ordering: the chain identifier, the entitlement contract, the source entitlement, the target entitlement, the old interval, the new interval, the seller, then the buyer; the first refusal returns and every later form is not built; `deps` and `params` carry nothing and are not read; no side effect
    * `[ ]`   Own entry `every_chain_forms_concrete_rebuilds_each_form_of_a_context_from_its_canonical_value`: condition `admit_contexts` is called, inside the consumer a concrete of `CHAIN_FORMS_CONCRETES` hands, with the canonical context at its builder defaults, whose chain identifier, entitlement contract, two entitlements, two intervals, seller, and buyer are of the kinds the concrete's forms declare and are pairwise distinct per field; outcome `Ok` whose context holds, in each of the eight form fields, the form whose `to_field` returns the like-named canonical value of the context, and whose suite identifier, asset root, parameter-set identifier, and expiry equal the canonical context's; variation each member of `CHAIN_FORMS_CONCRETES`, paired with the member of `DECLARED_ENCODING_SELECTIONS` its declaration requires; edge that must survive the composition: a value placed in another field's position, the source for the target, the old interval for the new, the seller for the buyer, or the entitlement contract for the seller, a form rebuilt from a default, and a form built by another form's type differ from the value built in Arrange
    * `[ ]`   Own entry `a_refused_form_names_the_field_that_held_its_value_over_every_chain_forms_concrete`: condition `admit_contexts` is called, inside the consumer a concrete of `CHAIN_FORMS_CONCRETES` hands, with the canonical context at its builder defaults except one form-typed field, which holds a `CanonicalFieldValue::Text`, a kind no form declares; outcome `Err(AdmitContextsErrorReturn::ChainIdentifier(FromFieldErrorReturn::WrongKind { expected }))` for the chain identifier and `Err(AdmitContextsErrorReturn::Field { field, error: FromFieldErrorReturn::WrongKind { expected } })` for each other field, `expected` the `KIND` of the form type the field names; variation each of the eight fields in turn, over each member of `CHAIN_FORMS_CONCRETES`; edge that must survive the composition: a refusal returned with another field's name, with the kind of another form, or as a different variant than the field's differs from the error built in Arrange
    * `[ ]`   Callee entries `chain_forms_identity_survives_its_field_round_trip`, `chain_forms_entitlement_survives_its_field_round_trip`, `chain_forms_interval_survives_its_field_round_trip`, and `chain_forms_chain_identifier_survives_its_field_round_trip` of `chain/create_chain_forms`: absorbed, proven by the `integration_test.rs` block `every_chain_forms_concrete_rebuilds_each_form_of_a_context_from_its_canonical_value`, whose run over `CHAIN_FORMS_CONCRETES` reaches `F::Identity`, `F::Entitlement`, `F::Interval`, and `F::ChainIdentifier` through `admit_contexts` and reads each admitted form back through `to_field`
    * `[ ]`   The remaining entries of `chain/create_chain_forms` concern the declaration the forms hand a consumer, the encoder admission, the key identifier, and the signed intent, none of which `admit_contexts` calls; the entries of `encoding/factory` concern the encoding concretes' handing to a consumer, those of `proof/factory` the admission of a delivery-proof concrete, those of `harness-crypto/generate/solidity/challenge_vectors` the challenge records, and those of `harness-crypto/config/admit_statements` the admission of the document's text, and no entry of them passes through `admit_contexts` or ends at its tests
    * `[ ]`   Private surface: the chain of real functions `create_chain_forms_case`, `create_encoding` with the encoding concrete the case params select, `create_chain_forms`, the selected forms concrete's producer, `consume_chain_forms`, `admit_contexts`, and each form's `from_field` and `to_field`, all running for real over the real declared sets; the outer-edge collaborator is the encoding family's real Ethereum ABI concrete; the observable result is the in-place assertions of the consumer on the `AdmitContextsReturn`; the entries proven are `every_chain_forms_concrete_rebuilds_each_form_of_a_context_from_its_canonical_value` and `a_refused_form_names_the_field_that_held_its_value_over_every_chain_forms_concrete`
    * `[ ]`   Public surface: an outside caller invokes `admit_contexts` through the crate root with an `AdmitContextsDeps`, an `AdmitContextsParams`, and an `AdmitContextsPayload` holding a `CanonicalDeliveryContext` and a purpose, from inside the `IChainFormsConsumer` it hands `chain`'s `create_chain_forms_case` through the chain crate's public surface under the `mocks` feature, and observes the `AdmitContextsReturn`, the forms in its success through `ICanonicalField::to_field`, and its refusal; the entries proven are the two own entries above, each over `CHAIN_FORMS_CONCRETES`; the outer-edge collaborator is the encoding family's real Ethereum ABI concrete

  * `[ ]`   `apps/harness-crypto/src/run/admit_contexts/mock.rs`
    * `[ ]`   Module-level `#![allow(dead_code, clippy::too_many_arguments)]`, the file holding no `unwrap`, `expect`, `panic!`, or numeric `as`
    * `[ ]`   `AdmitContextsDepsOverrides` and `AdmitContextsParamsOverrides`, each the fieldless struct with `#[derive(Default)]`; `build_admit_contexts_deps(_overrides) -> AdmitContextsDeps` and `build_admit_contexts_params(_overrides) -> AdmitContextsParams`
    * `[ ]`   `AdmitContextsPayloadOverrides`, the fieldless struct with `#[derive(Default)]`; `build_admit_contexts_payload<'a, Purpose>(context: &'a CanonicalDeliveryContext, purpose: Purpose, _overrides) -> AdmitContextsPayload<'a, Purpose>`, the handed context and purpose its leading parameters typed exactly as the fields, since no default can produce either
    * `[ ]`   `AdmitContextsSuccessReturnOverrides`, the fieldless struct with `#[derive(Default)]`; `build_admit_contexts_success_return<F: IChainForms, Purpose>(context: DeliveryContext<F, Purpose>, _overrides) -> AdmitContextsSuccessReturn<F, Purpose>`, the handed context its leading parameter
    * `[ ]`   `mock_admit_contexts<F: IChainForms, Purpose>(_deps: &AdmitContextsDeps, _params: AdmitContextsParams, payload: AdmitContextsPayload<'_, Purpose>) -> AdmitContextsReturn<F, Purpose>`, the production signature and nothing else, returning `Ok(build_admit_contexts_success_return(build_delivery_context::<F, Purpose>(payload.purpose, Default::default()), Default::default()))`, the context produced through the production bound `IChainForms` by the proof family's builder
    * `[ ]`   No corruptions type and no invalidator, since no value this interface owns arrives as untrusted data
    * `[ ]`   Dispositions: `AdmitContextsErrorReturn` and `DeliveryContextField`, enums, used by their production variants; `AdmitContextsReturn`, an alias of `Result`, and `AdmitContextsFn`, a function type whose mock is `mock_admit_contexts`; `AdmitContextsDeps`, `AdmitContextsParams`, `AdmitContextsPayload`, and `AdmitContextsSuccessReturn`, object types `interface.rs` declares, each with its builder above; no case runner, since the function's deps hold the types of one family, the chain family, whose own factory call and case runner reach the forms
    * `[ ]`   Imports this module's types from `super::interface`, `CanonicalDeliveryContext` from `crate::config::provides`, `DeliveryContext`, `build_delivery_context` from `proof`, and `IChainForms` from `chain`; nothing else

  * `[ ]`   `apps/harness-crypto/src/run/admit_contexts/test_basic.rs`
    * `[ ]`   Module-level `#![allow(clippy::expect_used)]`; imports `admit_contexts`, every type of `super::interface`, and every builder and overrides type of `super::mock` its blocks name; contains only imports, contract headers, blocks, and assertions
    * `[ ]`   Every block reaches `admit_contexts` through `create_chain_forms_case`: a local `ICreateChainFormsCaseConsumer` holds the block's arrangement, and its method builds deps with `build_create_chain_forms_deps(consumer, payload.encoder, Default::default())` over a local `IChainFormsConsumer` whose `consume_chain_forms<F: IChainForms>` makes the block's single call to `admit_contexts::<F, Purpose>` with `build_admit_contexts_deps`, `build_admit_contexts_params`, and `build_admit_contexts_payload` over the block's context and purpose and asserts on the return in place, so its `Output` is `()`; the block calls `create_chain_forms_case` over `build_create_chain_forms_case_deps`, `build_create_chain_forms_case_params`, and `build_create_chain_forms_case_payload` at their defaults, which select the family's mock forms over the mock encoding concrete
    * `[ ]`   The context of every block is `build_canonical_delivery_context` at its defaults unless the block names an override; expected values are the canonical values the context holds, each form read back through `to_field` over `build_to_field_params(Default::default())`, and are never read from the subject's own `from_field`
    * `[ ]`   `admit_contexts_rebuilds_the_chain_identifier_from_its_canonical_value`
      * `[ ]`   Contract: Chain identifier → the chain identifier form is built from the context's `chain`
      * `[ ]`   Arrange: the default context and `MintPurpose::Mint`
      * `[ ]`   Act: `admit_contexts`, once, inside the consumer
      * `[ ]`   Assert: the result is `Ok`; `to_field` over its `chain` returns `CanonicalFieldValue::Unsigned256([0x21; 32])`
    * `[ ]`   `admit_contexts_rebuilds_the_entitlement_contract_apart_from_the_seller_and_the_buyer`
      * `[ ]`   Contract: Entitlement contract → the entitlement contract form is built from `entitlement_contract`, not from `seller` or `buyer`
      * `[ ]`   Arrange: the default context
      * `[ ]`   Act: `admit_contexts`, once
      * `[ ]`   Assert: `to_field` over `entitlement_contract` returns `CanonicalFieldValue::FixedBytes20([0x31; 20])`, over `seller` `FixedBytes20([0x61; 20])`, and over `buyer` `FixedBytes20([0x62; 20])`
    * `[ ]`   `admit_contexts_rebuilds_the_source_and_target_entitlements_each_from_its_own_value`
      * `[ ]`   Contract: Entitlements → the source form is built from `source_entitlement` and the target form from `target_entitlement`
      * `[ ]`   Arrange: the default context
      * `[ ]`   Act: `admit_contexts`, once
      * `[ ]`   Assert: `to_field` over `source_entitlement` returns `Unsigned256([0x41; 32])` and over `target_entitlement` `Unsigned256([0x42; 32])`
    * `[ ]`   `admit_contexts_rebuilds_the_old_and_new_intervals_each_from_its_own_value`
      * `[ ]`   Contract: Intervals → the old form is built from `old_interval` and the new form from `new_interval`
      * `[ ]`   Arrange: the default context
      * `[ ]`   Act: `admit_contexts`, once
      * `[ ]`   Assert: `to_field` over `old_interval` returns `Unsigned64(1)` and over `new_interval` `Unsigned64(2)`
    * `[ ]`   `admit_contexts_carries_the_domain_values_of_the_context_unchanged`
      * `[ ]`   Contract: Domain values → the suite identifier, the asset root, and the parameter-set identifier are the canonical context's
      * `[ ]`   Arrange: the context from `build_canonical_delivery_context` overriding `suite_identifier` with `build_suite_identifier` over identifier bytes each `0x55` and version `7`, `asset_root` with `build_root` over bytes each `0x56`, and `parameter_set_identifier` with `build_parameter_set_identifier` over bytes each `0x57`
      * `[ ]`   Act: `admit_contexts`, once
      * `[ ]`   Assert: the admitted `suite_identifier`, `asset_root`, and `parameter_set_identifier` equal the three built in Arrange by `assert_eq!`
    * `[ ]`   `admit_contexts_carries_the_expiry_of_the_context_unchanged`
      * `[ ]`   Contract: Expiry → `expiry` is carried as the canonical context holds it
      * `[ ]`   Arrange: the context overriding `expiry` with `42`
      * `[ ]`   Act: `admit_contexts`, once
      * `[ ]`   Assert: the admitted `expiry` equals `42`
    * `[ ]`   `admit_contexts_carries_each_mint_relation_purpose_as_handed`
      * `[ ]`   Contract: Purpose → a mint context carries the purpose handed, whichever of the relation's purposes it is
      * `[ ]`   Arrange: the default context with `MintPurpose::Mint`, then with `MintPurpose::Replacement`
      * `[ ]`   Act: `admit_contexts`, once per purpose
      * `[ ]`   Assert: the admitted `purpose.code()` equals `1` and `4`
    * `[ ]`   `admit_contexts_carries_each_transfer_relation_purpose_as_handed`
      * `[ ]`   Contract: Purpose → a transfer context carries the purpose handed, whichever of the relation's purposes it is
      * `[ ]`   Arrange: the default context with `TransferPurpose::Transfer`, then with `TransferPurpose::Grant`
      * `[ ]`   Act: `admit_contexts`, once per purpose
      * `[ ]`   Assert: the admitted `purpose.code()` equals `2` and `3`
    * `[ ]`   `admit_contexts_does_not_alter_the_canonical_context_it_borrows`
      * `[ ]`   Contract: the borrow → the canonical context is unchanged after the call
      * `[ ]`   Arrange: the default context and a clone of it
      * `[ ]`   Act: `admit_contexts`, once
      * `[ ]`   Assert: the borrowed context equals the clone by `assert_eq!`

  * `[ ]`   `apps/harness-crypto/src/run/admit_contexts/test_error.rs`
    * `[ ]`   Module-level `#![allow(clippy::expect_used)]`; imports as `test_basic.rs` imports, with `AdmitContextsErrorReturn`, `DeliveryContextField`, `FromFieldErrorReturn`, and `CanonicalFieldKind`; contains only imports, contract headers, blocks, and assertions
    * `[ ]`   Every block reaches `admit_contexts` as `test_basic.rs` states and asserts the whole error in place with `assert_eq!` over `result.err()`, the refusal type deriving `PartialEq` and `Debug`; a field is made to refuse by overriding it in `build_canonical_delivery_context` with `CanonicalFieldValue::Text("not a field of this form".to_string())`, a kind no form declares
    * `[ ]`   `admit_contexts_refuses_a_chain_identifier_of_another_kind`
      * `[ ]`   Contract: Chain identifier → a refused chain identifier is returned as `AdmitContextsErrorReturn::ChainIdentifier(error)` with the form's refusal unchanged
      * `[ ]`   Arrange: the context overriding `chain` with the text value
      * `[ ]`   Act: `admit_contexts`, once
      * `[ ]`   Assert: `result.err()` equals `Some(AdmitContextsErrorReturn::ChainIdentifier(FromFieldErrorReturn::WrongKind { expected: CanonicalFieldKind::Unsigned256 }))`
    * `[ ]`   `admit_contexts_names_the_field_that_held_a_value_of_another_kind`
      * `[ ]`   Contract: each field other than the chain identifier → the refusal is `AdmitContextsErrorReturn::Field { field, error }` with the field's variant and the form's own expected kind
      * `[ ]`   Arrange: for each of `EntitlementContract`, `SourceEntitlement`, `TargetEntitlement`, `OldInterval`, `NewInterval`, `Seller`, and `Buyer`, the default context with that field alone overriding to the text value
      * `[ ]`   Act: `admit_contexts`, once per field
      * `[ ]`   Assert: each `result.err()` equals `Some(AdmitContextsErrorReturn::Field { field, error: FromFieldErrorReturn::WrongKind { expected } })` with `expected` `FixedBytes20` for `EntitlementContract`, `Seller`, and `Buyer`, `Unsigned256` for `SourceEntitlement` and `TargetEntitlement`, and `Unsigned64` for `OldInterval` and `NewInterval`
    * `[ ]`   `admit_contexts_returns_the_first_refusal_in_transcript_order`
      * `[ ]`   Contract: Ordering → the first refused field in transcript order returns and the later fields are not built
      * `[ ]`   Arrange: the default context overriding to the text value, in turn, `chain` and `buyer`; `entitlement_contract` and `seller`; `source_entitlement` and `new_interval`; `target_entitlement` and `old_interval`; `old_interval` and `buyer`; and `seller` and `buyer`
      * `[ ]`   Act: `admit_contexts`, once per context
      * `[ ]`   Assert: the results equal the refusals of `chain` as `ChainIdentifier`, `EntitlementContract`, `SourceEntitlement`, `TargetEntitlement`, `OldInterval`, and `Seller`, each with the form's own expected kind

  * `[ ]`   `apps/harness-crypto/src/run/admit_contexts/mod.rs`
    * `[ ]`   Module wiring: `mod interface;`, `pub mod delivery_context_field;`, `#[cfg(any(test, feature = "mocks"))] mod mock;`, `#[cfg(test)] mod test_basic;`, `#[cfg(test)] mod test_error;`, `#[cfg(test)] mod integration_test;`, and `pub mod provides;`; the implementation of `admit_contexts`, the trusted form, following the interaction spec's branches in its order; the file holds no `unwrap`, `expect`, `panic!`, or numeric `as`
    * `[ ]`   Each form is built by the form's `from_field` the spec names, each refusal is carried unchanged inside the variant of the field that held the value, and the context is assembled from the forms, the canonical context's domain values and expiry, and the purpose
    * `[ ]`   Introduces no undeclared dependency; bypasses no contract

  * `[ ]`   `apps/harness-crypto/src/run/admit_contexts/integration_test.rs`
    * `[ ]`   Module-level `#![allow(clippy::expect_used)]`; a `#[cfg(test)]` module of its own, never part of the unit test files; each block's `Contract` transcribes one entry of the spec's private surface and its `Arrange` names the entry's variation; each block's body is written once and run over `CHAIN_FORMS_CONCRETES`, so the test names no concrete and a further concrete adds a case without an edit
    * `[ ]`   Each block reaches `admit_contexts` through `create_chain_forms_case` as `test_basic.rs` states, pairing each member of `CHAIN_FORMS_CONCRETES` selected through `build_create_chain_forms_params` overriding `concrete` with the case params from `build_create_chain_forms_case_params` overriding `encoding` with the member of `DECLARED_ENCODING_SELECTIONS` that member's declaration requires
    * `[ ]`   `every_chain_forms_concrete_rebuilds_each_form_of_a_context_from_its_canonical_value`, one block, proving the own entry of the same name; Arrange for each member of `CHAIN_FORMS_CONCRETES` the default context and `MintPurpose::Mint`; Act `admit_contexts`, once per member, inside the consumer; Assert the result is `Ok`, `to_field` over each of the eight admitted forms returns the like-named canonical value of the context, each form's `KIND` equals the kind of the value it was built from, and the suite identifier, asset root, parameter-set identifier, and expiry equal the context's; Boundary `create_chain_forms_case`, `create_encoding`, `create_chain_forms`, the selected concrete's producer, `consume_chain_forms`, `admit_contexts`, and each form's `from_field` and `to_field`; Mocked none
    * `[ ]`   `a_refused_form_names_the_field_that_held_its_value_over_every_chain_forms_concrete`, one block, proving the own entry of the same name; Arrange for each member of `CHAIN_FORMS_CONCRETES` and each of the eight fields, the default context overriding that field alone to `CanonicalFieldValue::Text("not a field of this form".to_string())`; Act `admit_contexts`, once per member and field, inside the consumer; Assert `result.err()` equals `Some(AdmitContextsErrorReturn::ChainIdentifier(FromFieldErrorReturn::WrongKind { expected }))` for the chain identifier and `Some(AdmitContextsErrorReturn::Field { field, error: FromFieldErrorReturn::WrongKind { expected } })` for each other field, `expected` read as the `KIND` of the form type the field names; Boundary and Mocked as the block above states
    * `[ ]`   Reports its enumeration: every entry the private surface lists and the block that proves it

  * `[ ]`   `apps/harness-crypto/src/run/admit_contexts/provides.rs`
    * `[ ]`   `pub use super::interface::*;`, `#[cfg(any(test, feature = "mocks"))] pub use super::mock::*;`, `pub use super::delivery_context_field::provides::*;`, and the function's re-export `pub use super::admit_contexts;`; the module's only `pub mod` is `provides`

  * `[ ]`   `apps/harness-crypto/src/run/provides.rs`
    * `[ ]`   `pub use super::admit_contexts::provides::*;`, the module's surface, which the crate barrel re-exports, so the function and the moved type are public entries of the crate; a later function of the module adds its own re-export line

  * `[ ]`   `apps/harness-crypto/tests/admit_contexts_integration_test.rs`
    * `[ ]`   The public integration test, consuming only the crate's public surface and the official mocks under the `mocks` feature; each block transcribes one entry of the spec's public surface, its `Boundary` the public call `admit_contexts` through the crate root with the route of the entry and its `Mocked` the outer-edge collaborators on that route; module-level `#![allow(clippy::expect_used)]`
    * `[ ]`   `every_chain_forms_concrete_rebuilds_each_form_of_a_context_from_its_canonical_value` and `a_refused_form_names_the_field_that_held_its_value_over_every_chain_forms_concrete`: Contract the own entry of the same name as an outside caller observes it; Arrange as the private block states, through `harness_crypto` and the chain and encoding crates' public surfaces under their `mocks` features; Act `admit_contexts` through the crate root once per member and field, inside the consumer the chain crate's `create_chain_forms_case` hands; Assert as the private block states; Boundary the public call `admit_contexts` and the chain crate's public `create_chain_forms_case`, `create_chain_forms`, and the real forms; Mocked none
    * `[ ]`   Reports its enumeration: every entry the public surface lists and the block that proves it

  * `[ ]`   `directionality`
    * `[ ]`   `admit_contexts` depends on the config module's `CanonicalDeliveryContext` through `crate::config::provides`, on the `proof`, `chain`, `encoding`, `domain`, and `hashing` crates through their public surfaces, and on the standard library; no adapter crate names `apps/harness-crypto`; the module's surface leaves through `run::provides` and the crate barrel; `challenge_vectors` imports nothing of this module and this module imports nothing of `challenge_vectors`; the files that name `DeliveryContextField` through the crate root reach it through the barrel's re-export of this module; nothing under `delivery`, `vectors`, `verifier`, `measure`, `report`, `benchmark`, or `config` names this module's function; no cycle
    * `[ ]`   `harness-crypto/run/chain_forms_consumer` reads `admit_contexts` and `DeliveryContextField` from this node, and every module that takes a typed `DeliveryContext` takes the value this function returns

  * `[ ]`   `requirements`
    * `[ ]`   `apps/harness-crypto/src/lib.rs` gains the module and re-export lines its element states; `apps/harness-crypto/src/generate/solidity/challenge_vectors/interface.rs` loses the declaration of `DeliveryContextField` and no other item; `apps/harness-crypto/Cargo.toml`, `deny.toml`, `harness-crypto/vectors/statement_mutations`' files, and every file under `apps/harness-crypto/src/config`, `delivery`, `vectors`, `verifier`, `measure`, `report`, and `benchmark` are unchanged
    * `[ ]`   `cargo check --workspace --all-targets --all-features`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --all --check`, and `cargo deny check` complete without error or warning, the `admit_contexts` module having no unused item and no file naming `DeliveryContextField` through a path the move removed
    * `[ ]`   Every block of `test_basic.rs` passes: each form rebuilt from its own canonical value, the entitlement contract apart from the seller and the buyer, the two entitlements and the two intervals each from its own value, the domain values and the expiry carried, each relation's purposes carried as handed, and the borrowed context unaltered
    * `[ ]`   Every block of `test_error.rs` passes, each refusal returned whole with the chain identifier's own variant or the field's `DeliveryContextField` and the form's expected kind, and the first refusal in transcript order
    * `[ ]`   `every_chain_forms_concrete_rebuilds_each_form_of_a_context_from_its_canonical_value` and `a_refused_form_names_the_field_that_held_its_value_over_every_chain_forms_concrete` pass in `integration_test.rs` (each form of a context rebuilt over every concrete the declared set holds, and a refused form named with the field that held its value)
    * `[ ]`   Every block of `apps/harness-crypto/tests/admit_contexts_integration_test.rs` passes (the contexts admitted through the crate's public surface)
    * `[ ]`   Every test under `apps/harness-crypto/src/config`, `apps/harness-crypto/src/vectors`, `apps/harness-crypto/src/generate`, `apps/harness-crypto/src/delivery`, `apps/harness-crypto/src/verifier`, `apps/harness-crypto/src/measure`, `apps/harness-crypto/src/report`, and `apps/harness-crypto/src/benchmark` passes, and every test of the chain, encoding, proof, domain, and hashing crates passes

* `[ ]`   `harness-crypto/run/generate_consumer` 

* `[ ]`   `harness-crypto/run/verifier_consumer`

* `[ ]`   `harness-crypto/run/delivery_proof_consumer` 

* `[ ]`   `harness-crypto/run/key_agreement_consumer` 

* `[ ]`   `harness-crypto/run/kem_consumer` 

* `[ ]`   `harness-crypto/run/chain_forms_consumer` 

* `[ ]`   `harness-crypto/run/encoding_consumer`

* `[ ]`   `harness-crypto/run/pairing_consumer`

* `[ ]`   `harness-crypto/run` 

* `[ ]`   `harness-crypto/output`

* `[ ]`   `harness-crypto/main`

* `[ ]`   `contracts/evm/PairingLib`

* `[ ]`   `contracts/evm/DeliveryVerifier`

* `[ ]`   `contracts/evm/deploy`
