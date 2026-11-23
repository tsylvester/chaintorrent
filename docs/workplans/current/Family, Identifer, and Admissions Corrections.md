* `[ ]`   `incomplete item`

* `[✅]`   `complete item`

# Family Identifier and Admission Correction Plan

Read `docs/agents/index.md`. Every time during your turn you need to do something that has a rules topic, read the rules topic before taking the action, and ensure your action is compliant with the rules topic. 

## Purpose of Correction

### Family Identifier and Admission

Values that name members of closed sets must be carried as typed identifiers rather than text. Text is admitted only at an input boundary and rendered only at an output boundary. Declarations must not carry duplicate text names, and diagnostic renderings must not be used as names, keys, comparisons, or document values.

Each family must read member declarations through its declaration lookup, turn configured identifiers and settings into selections through family and concrete admission, and admit composition before construction or effort. A concrete has a settings module only where configuration may set a value it is constructed from, that module precedes the concrete, and a value the concrete fixes is a value it provides, never a setting. A value multiple families must agree on has one identifier type and one spelling, owned by the family whose members provide it; every consuming family depends on that family for the type, states its requirements in that type's values rather than by naming another family's members, and never enumerates the value's members or holds its own copy of the type. Internal records, map keys, parameters, payloads, comparisons, and refusals must preserve those typed identities.

### Chain/Forms Family Delineation

A family is one interface and the concretes that implement it, with its own selection, declaration lookup, selection admission, composition admission, and factory. The chain forms and the chain adapters are two families: the forms implement `IChainForms` and are consumed by the proof family's transcript descriptions, challenge, concrete, and factory, the generator, the verifier client, and the measurement; the chain adapters implement the chain interface and are consumed by the verifier client, the configuration, and the run. The planning documents and workplans hold them as one chain family. The forms concrete `chain/evm_forms` and the chain concrete `chain/base` share one set of family functions, and the forms are constructed by a family-owned function, `chain/create_chain_forms`, standing in for a factory because the chain factory module holds the function that constructs the chain concrete. The proof tickets need the forms before `chain/base` exists, and `chain/base` depends on the proof factory, so no single position serves the shared functions: `chain/lookup_chain_declaration`, `chain/admit_chain_selection`, and `chain/admit_chain_composition` are each authored ahead of `chain/create_chain_forms` and again as a Base increment ahead of `chain/factory`, within one milestone, which is several nodes for one function in a milestone. The shared selection admission receives "the admitted member or forms identifier", and the composition admission's Base increment is not a location a further concrete changes. The dependency map states the split; this correction aligns every other planning document and every workplan with it, so each family function is authored once, in its own family.

## The Correction to be Made

### Family Identifier and Admission

- Every closed-set member is represented by its owning identifier type with one spelling per member, total over the set and declared beside the type, and, where a wire encoding exists, one wire code per member, total over the set and declared beside the type. A family owns a member identifier naming its members and a value identifier for each value its members provide that another family compares or that configuration names; a value identifier has a variant only for a value a real member of the declared set provides. Declared sets contain real members and exclude mock-only members. Between the input and output boundaries, record fields, map keys, params, payload, refusals, and comparisons carry identifiers, never text.
- Input text is admitted by the identifier's owner against the spelling of each declared real member, with unknown text refused by a typed error carrying that text. A family's selection admission receives the admitted identifier and the untrusted settings document. For a member with a settings module it narrows that document through the settings module's guard, which refuses a key naming no field of the settings type, invokes the settings admission, which refuses a value outside its declared bounds and takes the declared default where the document supplies none, and returns the selection carrying the admitted settings; for a member with no settings module it refuses a document that supplies any setting, with the family's own error naming the member, and returns the selection naming the member. Every callee error is carried unchanged.
- A concrete has a settings module only where configuration may set a value it is constructed from. The settings module owns the concrete's settings type, its settings-document type and the guard that narrows to it, and the settings admission, with the settings type's builder in the settings module's mock file. The concrete's declared settings state, for each settable field, its bounds and its default; a value the concrete fixes is stated as a value the concrete provides, never as a setting. The settings module precedes its concrete in dependency order and, as the first consumer of the concrete's declaration constant, authors it in the concrete's interface; the family's selection admission follows both. A concrete with no settable value has no settings module, no settings node, no settings table, and a selection variant carrying no settings, and its declared-selection builder arm returns the selection with no configuration.
- A declaration lookup returns a member's declaration without constructing the member. A declaration states what its member provides in its family's value identifiers, and what it requires of each family it is composed with in that family's value identifiers, never by naming another family's member. A composition admission compares the selected member's declared requirements with the declarations of the members it is composed with and returns the family's `Incompatible` flavor where they disagree. Its factory calls the composition admission before constructing anything, and a composition performing side effects completes every required admission before its first side effect.
- A value families must agree on is declared once, with its spelling, in the family whose members provide it, and exported through that family's `provides`. Each consuming family depends on the providing family for the type and its declarations carry that shared identifier; the dependency runs from consumer to provider and forms no cycle. No module outside the providing family enumerates the value's members, declares a copy of its type, or holds it as text, and a value no family's members provide is declared in the module that implements it. A declaration carries its member's identifier and no duplicate text name.
- A report, vector record, file, path, log, or other output document renders an identifier's spelling or its declared wire code only at the output boundary. `Debug` and `Display` text are never admitted, compared, keyed, or written as names.
- Producers are corrected before their consumers, and a providing family's value identifier is corrected before every family that consumes it. Existing nodes are revised in place at their dependency positions, and missing source functions receive complete nodes at the position between their last producer and first consumer.

### Chain/Forms Family Delineation

The correction separates one conflated family into two and moves every statement to the family it belongs to. A document is defective wherever it treats the forms as members, functions, types, or declarations of the chain family, and wherever it carries a workaround that exists only because of that conflation.

**Recognizing the conflation.** The forms and the chain adapters differ in every respect that defines a family: they implement different interfaces, are selected by different identifiers, are constructed by different functions, and are consumed at different dependency positions, the forms ahead of the proof family and the chain adapters after it. Each of the following is a symptom of holding them as one family, and each is corrected by the split rather than reworded:
- a selection or selection admission that receives a member identifier or a forms identifier;
- a family-owned function that constructs a concrete because the family's factory constructs a different one, and every justification that the factory module "holds one function";
- a family function authored twice in one milestone, once ahead of the forms constructor and again as an increment for the chain concrete;
- a declaration split into a "forms portion" and a "remainder", or interfaces said to wait for a later concrete of the same family;
- a forms concrete described as the chain family's first concrete, or a forms settings module.

**Where each statement goes.** A statement about the forms — their interface, their identifier, their declaration, their mock, their construction over a borrowed encoder, the encoding identifier they require, the refusal of settings they do not have, address derivation, and the signed-intent form — belongs to the chain forms family. A statement about the chain adapters — settlement and entitlement state, the chain declaration and its settings, the member identifier, the attempt context, the estimated fee — belongs to the chain family. The chain family's first ticket is therefore the first ticket that needs the chain family's own types, and it creates the chain crate; nothing about the chain family is authored by the forms concrete.

**How the families relate.** The chain family consumes the chain forms family, one way. A chain member binds forms: its declaration states the forms it binds as a requirement in the chain forms family's identifier, and the chain family's composition admission compares that requirement with what the resolved forms declare. Neither family names the other's members. A value neither family provides is compared by the family that provides it, so a precompile encoding is compared by the pairing family's composition admission and by no chain or forms admission.

**What becomes of the duplicated functions.** Each family authors its own declaration lookup, selection admission, composition admission, and factory once. The functions placed ahead of the forms constructor are the chain forms family's functions, carrying only what concerns the forms. The functions placed after the chain concrete, together with their increments, are the chain family's functions, each one node, carrying only what concerns the chain adapters. The forms constructor is the chain forms family's factory, described as every factory is, its callers reaching it by the name every factory's callers use for their family.

**How consumers change.** A consumer depends on the surface of the family whose value it uses: a consumer of forms on the chain forms family's surfaces, a consumer of a chain adapter on the chain family's, and a consumer that compares against both declarations on both lookups. The configuration admits the forms selection and the chain selection separately, each through its own family's selection admission.

**Names.** The chain forms family's crate, tickets, modules, functions, and types are named by the same schema as every other family's. No name of the conflated form remains in any document as an alias or as history.

## Order of the Required Correction

The specifications precede the planning documents, and the planning documents precede the workplans, because each layer states what the layer below it cites. Within the specification layer and the planning layer, a document is corrected after every document it cites. Within the workplans, the pre-WBS sections precede the nodes, and nodes are corrected in Work Breakdown Structure order, so every corrected node's producers are corrected before it. Each document and each node is corrected once, for every coupling it carries.

A node marked complete whose content changes returns to unbuilt, and its files are rebuilt from the corrected node. A node the corrected dependency map adds inside the span the workplans already author is inserted at its dependency position. A node whose content no coupling touches is unchanged.

Before the producer section or any correction item is populated, the read-only inventory establishes the actual producer graph and freezes every document, node, and insertion position. Once that order is frozen, the correction document is populated incrementally in that order, and every executable item carries its own required context.

## Document Writing Procedure

**How to work.**
- Pick up the first document that has no sub-items.
- Read the document.
- At each instance of a string or section that hosts an instance of the problem, create a new bullet under the document, and add an anchor to the start and end of the string or section that needs corrected. 
- Then give instructions for how to correct the string or section to comply with the correction to be made. 
- Increment through the document until all instances of the problem have been bulleted and have a method of correction.
- Review the document again to ensure that the bullets listed for the document are comprehensive, coherent, complete, and correct. 
- Halt and ask the user to review your work. 

## Correction Procedure

**How to work.**
- Find each node by its heading, the backticked node name at the start of a top-level bullet, in the document.
- Do not read forward. You will not benefit from swamping your context by trying to read everything at once. Read incrementally as you go. 
- Edit only the bullets named below and leave every other bullet as it is.
- Elements in a workplan node follow the status marking rules from `workplan-structure.md`.
- Where an item says "replace X with Y", X is quoted from the node exactly; find it and substitute Y word for word.
- Keep each bullet on one unbroken line.
- Put no rule names or explanations into node text. Write the text the item gives.
- Complete one item at a time. 
- In this document, mark each item completed when the relevant line in the workplan is updated. 
- Do not batch edits to any document. One edit per document per item. No lumps, blobs, or batches. 
- Move to the next item.
- Do not try to apply multiple edits from this document at once. Work one by one.  
- If you make a discovery, you must explain the discovery to the user following `discovery-halt.md`. 
- Proofs are explicitly labeled. If you haven't reached a labeled proof marker at the end of a group, you haven't completed the proofs. 
- Halt when you reach an item marked "Halt", when you complete the entire set of proofs for a group, or when you make a discovery.

## `docs/research` layer

Done

## `docs/project-planning` layer

Done

## Workplan layer

Each entry names its nodes in Work Breakdown Structure order and, per node, the elements that change. An element not named is unchanged. Within a named node, every restatement of a changed item in `objective`, `role`, `module`, `deps`, `context_slice`, and `requirements` is corrected with it.

### Workspace and discipline bootstrap (foundation)

* `[✅]`   `workspace/cargo`
* `[✅]`   `domain/secret`
* `[✅]`   `random/os`
    * `[✅]`   Start anchor: `random/os` **Operating-system randomness concrete, the one source every production draw passes through; creates the `adapters/random` crate and authors the randomness family's generic interface, declaration, and mock**; end anchor: `Outside: which values a caller draws and what it builds from them, the sampling of scalars from uniform bytes, and the factory's admission rule`. Correction: revise the node summary, objective, role, and module to state that the concrete authors the family's interface, member identifier with one total spelling per real member, declared set, declaration, selection, and mock; the real selection carries no settings, the mock selection alone carries its failure mode, and the operating-system concrete has no settings module, settings node, or settings table because it has no settable value.
    * `[✅]`   Start anchor: `adapters/random/src/factory/interface.rs`; end anchor: "Imports `domain::Secret` and `OsRandomSourceFillBytesErrorReturn` from `crate::os::provides`; names no vendor". Correction: keep `RandomSourceKind` as the typed kind carried by declarations and selections; add an exhaustive `name()` mapping with `OperatingSystem` spelled `operating_system` and, under the mocks feature, `Mock(_)` spelled `mock`; add `RANDOM_SOURCE_KINDS: &[RandomSourceKind]` containing only the real `OperatingSystem` member, so input admission accepts only declared real spellings and never mock text; keep `RandomSourceDeclaration.source` typed as `RandomSourceKind`, with no duplicate text name; remove `declaration(&self)` from `IRandomSourceAdapter`, with declaration consumers using `random/lookup_random_declaration` without constructing a member.
    * `[✅]`   Start anchor: `adapters/random/src/os/interface.rs`; end anchor: "A `payload.length` of zero takes the drawn branch with an empty buffer; `params` carries no control and is not read". Correction: declare `OsRandomSource::DECLARATION` in `os/interface.rs` as `RandomSourceDeclaration { source: RandomSourceKind::OperatingSystem, adapter_version: 1, interface_version: RANDOM_SOURCE_INTERFACE_VERSION }`; keep the constructor settings-free and state that this concrete has no settings module; revise the interaction specification so the declaration is the interface-held constant read through `random/lookup_random_declaration` without an instance, not a value obtained from a constructed adapter.
    * `[✅]`   Start anchor: `adapters/random/src/factory/mock.rs`; end anchor: "Imports `Secret` and `build_secret` from `domain`, `core::cell::Cell` and `core::convert::Infallible`, and this module's types from `super::interface`". Correction: remove the mock concrete's `declaration()` implementation with the trait method; add the fieldless `RandomSourceKindOverrides` and `build_random_source_kind(kind: RandomSourceKind, overrides: RandomSourceKindOverrides) -> RandomSourceKind` as the declared-selection builder over `RANDOM_SOURCE_KINDS`, with the real `OperatingSystem` arm returning the selection without configuration; replace the statement that `RandomSourceKind` has no builder, and keep the failure mode only in the mock selection's configuration, outside the declared real set.
    * `[✅]`   Start anchor: `os_random_source_declares_its_adapter_and_interface_versions`; end anchor: "the final `Assert` line of `declaration_returns_the_associated_declaration`". Correction: retain the test that reads `OsRandomSource::DECLARATION` without an instance and add coverage proving that `RANDOM_SOURCE_KINDS` contains only real members and that every declared member's `name()` spelling is distinct; remove `declaration_returns_the_associated_declaration`, which tests the eliminated instance API.
    * `[✅]`   Start anchor: `construction`; end anchor: "No other item; no `unsafe`, `unwrap`, `expect`, `panic!`, or numeric `as`". Correction: state that `OsRandomSource::try_new` remains the concrete's only producer and the factory remains its only caller, but consumers read the declaration through `random/lookup_random_declaration` without constructing the source; remove `DECLARATION` from `os/mod.rs` because it now lives in `os/interface.rs`, and remove the `declaration()` override so the trait implementation retains only `fill_bytes` beside the constructor.
    * `[✅]`   Start anchor: `requirements`; end anchor: "Code outside `adapters/random` naming `OsRandomSource` or anything under `os` fails to compile; the crate's public surface is the `factory` module's `provides`". Correction: retain the draw, error, lint, and privacy requirements; add requirements proving that `RANDOM_SOURCE_KINDS` contains only real members, each declared member has a distinct canonical spelling, `RandomSourceDeclaration.source` remains a typed identifier with no text duplicate, `OsRandomSource::DECLARATION` is readable without an instance, the concrete has no settings module or settings table, and `build_random_source_kind` returns the real selection without configuration; remove every requirement for `adapter.declaration()` or another instance-based declaration read.
* `[✅]`   `random/lookup_random_declaration` — Halt: This node must be authored
* `[✅]`   `random/admit_random_selection` — Halt: This node must be authored
* `[✅]`   `random/factory` — Halt: This node must be revised
* `[✅]`   `workspace/ci`

### Pairing adapters and key derivation (harness grouping)

* `[✅]`   `pairing/bn254_arkworks`
    * `[✅]`   Start anchor: the `pairing/bn254_arkworks` top-level heading; end anchor: the `module` element's `Outside` bullet. Correction: replace the concrete's text name with its typed `PairingConcrete` identifier throughout the objective, role, and module; state that the declaration carries that identifier with no duplicate text name, that member and value identifiers have total spellings over real declared sets, and that this node declares only the Bn254 arkworks member and its provided values, with further concretes extending those locations in their own increments; give real selections no settings and this settings-free concrete no settings module, node, or table; route declaration reads through `pairing/lookup_pairing_declaration` and composition admission before construction.
    * `[✅]`   Start anchor: the `PairingCurve`, `VerifierGroupArithmetic`, and `PrecompileEncoding` bullet in `factory/interface.rs`; end anchor: the `TargetGroupEncodingIdentifier` bullet. Correction: keep only real provided values in these value identifiers: `PairingCurve::Bn254` with spelling `bn254`, `VerifierGroupArithmetic::FirstGroupOnly` with spelling `first_group_only`, `PrecompileEncoding::Eip196Eip197` with spelling `eip196_eip197`, and `TargetGroupEncodingIdentifier::Bn254V1` with spelling `bn254_v1`; remove every mock-only value variant and remove `Bls12381V1` until its providing concrete's increment; add `TARGET_GROUP_ENCODING_IDENTIFIERS: &[TargetGroupEncodingIdentifier]` containing only `Bn254V1`, with later real values added by their providers.
    * `[✅]`   Start anchor: the `PairingConcrete` bullet in `factory/interface.rs`; end anchor: the `PairingDeclaration` bullet. Correction: limit `PairingConcrete` to the existing `Bn254Arkworks` member and, under the mocks feature, `Mock(MockIPairingAdapterFailureMode)`; add an exhaustive `name()` mapping with `Bn254Arkworks` spelled `bn254_arkworks` and `Mock(_)` spelled `mock`; add `PAIRING_CONCRETES: &[PairingConcrete]` containing only `Bn254Arkworks`, so later concretes are added only by their own increments; replace `concrete_name: &'static str` with `concrete: PairingConcrete`, making the declaration carry the typed member identifier and no duplicate text name.
    * `[✅]`   Start anchor: the `IPairingAdapter` bullet in `factory/interface.rs`; end anchor: that same bullet. Correction: remove `const DECLARATION` and `const CONCRETE` from the trait while preserving its associated types and methods; a member's declaration, including its `concrete` identifier, is read through `pairing/lookup_pairing_declaration` without constructing the member, not exposed as a second trait-constant surface.
    * `[✅]`   Start anchor: `adapters/pairing/src/bn254_arkworks/interface.rs`; end anchor: the `CONCRETE` branch in `bn254_arkworks/interaction.spec.md`. Correction: declare `Bn254ArkworksPairing::DECLARATION` in the concrete's interface with `concrete: PairingConcrete::Bn254Arkworks`, the real Bn254 curve, arithmetic, precompile, and target-group values, and the declared versions; keep the constructor settings-free and state that no settings module exists; revise the interaction spec to read that interface-held declaration through `pairing/lookup_pairing_declaration` without an instance, remove the separate `CONCRETE` section and every trait-constant reference, and delete the `concrete_name` literal.
    * `[✅]`   Start anchor: the `PairingDeclarationOverrides` and `build_pairing_declaration` bullet in `factory/mock.rs`; end anchor: the bullet beginning `No builder for the fieldless params and payloads`. Correction: replace the declaration override and default for `concrete_name` with `concrete: Option<PairingConcrete>` and `PairingConcrete::Bn254Arkworks`; add `MOCK_I_PAIRING_ADAPTER_DECLARATION` with `concrete: PairingConcrete::Mock(MockIPairingAdapterFailureMode::Succeeds)` and only real Bn254-provided curve, precompile, and target-group values, with the lookup substituting the selected mock identifier as it does for random; remove the mock's trait constants; add fieldless `PairingConcreteOverrides` and `build_pairing_concrete` over `PAIRING_CONCRETES`, whose real arm returns the selection without configuration; remove `PairingConcrete` from the no-builder statement while leaving ordinary value enums used directly.
    * `[✅]`   Start anchor: the compile-time assertion over `Bn254ArkworksPairing::DECLARATION`; end anchor: the compile-time assertion over `<Bn254ArkworksPairing as IPairingAdapter>::CONCRETE`. Correction: assert the interface-held declaration's `concrete` is `PairingConcrete::Bn254Arkworks` and every other field is its real provided value, with no text-name assertion; remove the trait `DECLARATION` and `CONCRETE` assertions because those constants no longer exist; add coverage proving `PAIRING_CONCRETES` and `TARGET_GROUP_ENCODING_IDENTIFIERS` contain only real currently provided members, their `name()` spellings are distinct, and no value identifier contains a mock-only variant.
    * `[✅]`   Start anchor: `construction`; end anchor: the final `bn254_arkworks/mod.rs` bullet. Correction: keep `Bn254ArkworksPairing::try_new` as the concrete's only producer, but have the factory obtain the declaration through `pairing/lookup_pairing_declaration` and complete `pairing/admit_pairing_composition` before calling it and before its correction computation; move `DECLARATION` out of `mod.rs` into the interface; remove the trait `DECLARATION` and `CONCRETE` implementations and adjust imports so the interface, not the trait, owns the real declaration.
    * `[✅]`   Start anchor: `directionality`; end anchor: the bullet beginning `pairing/bn254_halo2curves`, `pairing/bls12_381_arkworks`, and `pairing/bls12_381_halo2curves`. Correction: remove the premature enumeration of future concretes; state that `pairing/lookup_pairing_declaration` reaches each existing member's interface-held declaration through the family's private path, the factory receives declarations through that lookup and completes composition admission before construction, and each later concrete adds its own declaration, identifier variant, declared-set entry, and family-function branch in its own increment without changing consumers outside the family.
    * `[✅]`   Start anchor: `requirements`; end anchor: the final `requirements` bullet. Correction: retain the algebraic, encoding, zeroization, lint, and privacy proofs; add requirements proving the declaration carries `PairingConcrete` rather than text, `PAIRING_CONCRETES` and `TARGET_GROUP_ENCODING_IDENTIFIERS` contain only real currently provided values with distinct spellings, no value identifier has a mock-only variant, real selections have no settings and the concrete has no settings module or table, `build_pairing_concrete` returns the real selection without configuration, and the factory uses declaration lookup and composition admission before construction; remove the trait `DECLARATION`, trait `CONCRETE`, and `concrete_name` proofs.
* `[✅]`   `pairing/bn254_halo2curves`
    * `[✅]`   Start anchor: the `pairing/bn254_halo2curves` top-level heading; end anchor: the `module` element's `Outside` bullet. Correction: replace the concrete's text name with `PairingConcrete::Bn254Halo2curves` throughout the objective, role, and module; state that its declaration carries that typed identifier with no duplicate text name and the same real Bn254-provided values as the arkworks member, so no new value-identifier variant is needed; give this settings-free concrete no settings module, node, or table and its real selection no settings; require declaration lookup and composition admission before construction, and describe this node as the increment that adds its member variant, spelling, declared-set entry, and declared-selection builder arm.
    * `[✅]`   Start anchor: the `role` bullet that currently says `edits no file of the factory module`; end anchor: that same bullet. Correction: replace `edits no file of the factory module` with explicit increments to `factory/interface.rs` and `factory/mock.rs`, and add both files to the node's file list: add `PairingConcrete::Bn254Halo2curves`, its `name()` spelling `bn254_halo2curves`, and its entry in `PAIRING_CONCRETES` beside `Bn254Arkworks`, keeping the set real-only and ordered by the existing nodes; add the `build_pairing_concrete` arm returning this real selection without configuration; leave `PairingCurve`, `VerifierGroupArithmetic`, `PrecompileEncoding`, `TargetGroupEncodingIdentifier`, and `TARGET_GROUP_ENCODING_IDENTIFIERS` unchanged because this member provides no new value.
    * `[✅]`   Start anchor: the `deps` bullet naming `PairingDeclaration` and `PairingConcrete`; end anchor: that same bullet. Correction: name the corrected `PairingDeclaration` with its typed `concrete` field, `PairingConcrete` with `Bn254Halo2curves` and its `name()` mapping, `PAIRING_CONCRETES` with both real BN254 members, and `build_pairing_concrete` with this member's no-configuration arm under the mocks feature; keep the existing real Bn254 value identifiers and every trait and sampling-bound type and builder, without naming a settings module because this member has none.
    * `[✅]`   Start anchor: `adapters/pairing/src/bn254_halo2curves/interface.rs`; end anchor: the `CONCRETE` branch in `bn254_halo2curves/interaction.spec.md`. Correction: declare `Bn254Halo2curvesPairing::DECLARATION` in the concrete's interface with `concrete: PairingConcrete::Bn254Halo2curves`, the real Bn254 curve, first-group-only arithmetic, EIP-196/EIP-197 precompile encoding, `TargetGroupEncodingIdentifier::Bn254V1`, and the declared versions; keep the constructor settings-free and state that no settings module exists; revise the interaction spec to read that interface-held declaration through `pairing/lookup_pairing_declaration` without an instance, remove the separate `CONCRETE` section and every trait-constant reference, and delete the `concrete_name` literal.
    * `[✅]`   Start anchor: the compile-time assertion over `Bn254Halo2curvesPairing::DECLARATION`; end anchor: the compile-time assertion over `<Bn254Halo2curvesPairing as IPairingAdapter>::CONCRETE`. Correction: assert the interface-held declaration's `concrete` is `PairingConcrete::Bn254Halo2curves` and every other field is its real Bn254-provided value, with no text-name assertion; remove the trait `DECLARATION` and `CONCRETE` assertions because those constants no longer exist; add coverage proving `PAIRING_CONCRETES` now contains `Bn254Arkworks` and `Bn254Halo2curves` and no mock member, the new spelling is distinct, `TARGET_GROUP_ENCODING_IDENTIFIERS` remains real-only, and the new declared-selection arm returns no configuration.
    * `[✅]`   Start anchor: `construction`; end anchor: the final `bn254_halo2curves/mod.rs` bullet. Correction: keep `Bn254Halo2curvesPairing::try_new` as the concrete's only producer, but have the factory obtain this member's declaration through `pairing/lookup_pairing_declaration` and complete `pairing/admit_pairing_composition` before calling it; move `DECLARATION` out of `mod.rs` into the interface; remove the trait `DECLARATION` and `CONCRETE` implementations and adjust imports so the interface, not the trait, owns the declaration.
    * `[✅]`   Start anchor: `directionality`; end anchor: the bullet beginning `pairing/factory` constructs this concrete. Correction: replace `no concrete depends on it` with the actual family dependencies: `pairing/lookup_pairing_declaration` reaches this interface-held declaration through the family's private path, selection and composition admission consume the corrected `PairingConcrete` identifier and declared set, and the factory uses those admissions before constructing this member; keep the concrete independent of every other concrete, and state that its family-interface and mock increments introduce no dependency outside the pairing family.
    * `[✅]`   Start anchor: `requirements`; end anchor: the final `requirements` bullet. Correction: retain the algebraic, encoding, zeroization, lint, and privacy proofs; add requirements proving the declaration carries `PairingConcrete::Bn254Halo2curves` rather than text, `PAIRING_CONCRETES` contains both real BN254 members with distinct spellings and no mock member, `TARGET_GROUP_ENCODING_IDENTIFIERS` remains real-only, the new declared-selection arm returns no configuration, the concrete has no settings module or table, and the factory uses declaration lookup and composition admission before construction; remove the trait `DECLARATION`, trait `CONCRETE`, and `concrete_name` proofs.
* `[✅]`   `pairing/bls12_381_arkworks`
    * `[✅]`   Start anchor: the `pairing/bls12_381_arkworks` top-level heading; end anchor: the `module` element's `Outside` bullet. Correction: replace the concrete's text name with `PairingConcrete::Bls12381Arkworks` throughout the objective, role, and module; state that its declaration carries that typed identifier with no duplicate text name and the real BLS12-381 curve, both-group arithmetic, EIP-2537 precompile, and `Bls12381V1` target-group values this member provides; give this settings-free concrete no settings module, node, or table and its real selection no settings; require declaration lookup and composition admission before construction, and describe the node as the increment adding its member variant, all four new value variants and their spellings, both declared-set entries, and its declared-selection builder arm.
    * `[✅]`   Start anchor: the `role` bullet that currently adds three declaration variants and says `edits no other item of the factory module`; end anchor: that same bullet. Correction: retain `PairingCurve::Bls12381` with spelling `bls12381`, `VerifierGroupArithmetic::BothGroups` with spelling `both_groups`, and `PrecompileEncoding::Eip2537` with spelling `eip2537`; add `TargetGroupEncodingIdentifier::Bls12381V1` with spelling `bls12381_v1` and its entry in `TARGET_GROUP_ENCODING_IDENTIFIERS` beside `Bn254V1`; add `PairingConcrete::Bls12381Arkworks` with spelling `bls12381_arkworks` and its entry in `PAIRING_CONCRETES`; add the `build_pairing_concrete` arm returning this real selection without configuration; include `factory/mock.rs` in the node's files, leaving every mock-only value variant absent.
    * `[✅]`   Start anchor: `adapters/pairing/src/factory/interface.rs`; end anchor: the bullet holding `PairingCurve::Bls12381`, `VerifierGroupArithmetic::BothGroups`, and `PrecompileEncoding::Eip2537`. Correction: expand that element to carry `TargetGroupEncodingIdentifier::Bls12381V1` with spelling `bls12381_v1` and its entry in `TARGET_GROUP_ENCODING_IDENTIFIERS`, `PairingConcrete::Bls12381Arkworks` with spelling `bls12381_arkworks` and its entry in `PAIRING_CONCRETES`, and the `name()` mappings `bls12381`, `both_groups`, and `eip2537` for the other new values; keep every entry real and leave later members to their own increments.
    * `[✅]`   Start anchor: the `deps` bullet naming `PairingDeclaration` and `PairingConcrete`; end anchor: that same bullet. Correction: name the corrected `PairingDeclaration` with `concrete: PairingConcrete::Bls12381Arkworks`, `PairingConcrete` with its new variant and `name()` mapping, `PAIRING_CONCRETES` with all three real members now present, the new BLS curve, both-group arithmetic, EIP-2537, and `Bls12381V1` values with their spellings, `TARGET_GROUP_ENCODING_IDENTIFIERS` with both real encoding identifiers, and `build_pairing_concrete` with this member's no-configuration arm under the mocks feature; keep every trait and sampling-bound type and builder, without naming a settings module because this member has none.
    * `[✅]`   Start anchor: `adapters/pairing/src/bls12_381_arkworks/interface.rs`; end anchor: the `CONCRETE` branch in `bls12_381_arkworks/interaction.spec.md`. Correction: declare `Bls12381ArkworksPairing::DECLARATION` in the concrete's interface with `concrete: PairingConcrete::Bls12381Arkworks`, `PairingCurve::Bls12381`, `VerifierGroupArithmetic::BothGroups`, `PrecompileEncoding::Eip2537`, `TargetGroupEncodingIdentifier::Bls12381V1`, and the declared versions; keep the constructor settings-free and state that no settings module exists; revise the interaction spec to read that interface-held declaration through `pairing/lookup_pairing_declaration` without an instance, remove the separate `CONCRETE` section and every trait-constant reference, and delete the `concrete_name` literal.
    * `[✅]`   Start anchor: the compile-time assertion over `Bls12381ArkworksPairing::DECLARATION`; end anchor: the compile-time assertion over `<Bls12381ArkworksPairing as IPairingAdapter>::CONCRETE`. Correction: assert the interface-held declaration's `concrete` is `PairingConcrete::Bls12381Arkworks` and every other field is its real BLS12-381-provided value, with no text-name assertion; remove the trait `DECLARATION` and `CONCRETE` assertions because those constants no longer exist; add coverage proving `PAIRING_CONCRETES` contains all three real members, `TARGET_GROUP_ENCODING_IDENTIFIERS` contains both real encodings, every new spelling is distinct, and the new declared-selection arm returns no configuration.
    * `[✅]`   Start anchor: `construction`; end anchor: the final `bls12_381_arkworks/mod.rs` bullet. Correction: keep `Bls12381ArkworksPairing::try_new` as the concrete's only producer, but have the factory obtain this member's declaration through `pairing/lookup_pairing_declaration` and complete `pairing/admit_pairing_composition` before calling it and before its correction computation; move `DECLARATION` out of `mod.rs` into the interface; remove the trait `DECLARATION` and `CONCRETE` implementations and adjust imports so the interface, not the trait, owns the declaration.
    * `[✅]`   Start anchor: `directionality`; end anchor: the bullet beginning `pairing/factory` constructs this concrete. Correction: replace `no concrete depends on it` and `the factory module depends on no concrete` with the actual family dependencies: `pairing/lookup_pairing_declaration` reaches this interface-held declaration through the family's private path, selection and composition admission consume the new member and value identifiers, and the factory uses those admissions before constructing this member; keep the concrete independent of every other concrete, and state that its family-interface and mock increments introduce no dependency outside the pairing family.
    * `[✅]`   Start anchor: `requirements`; end anchor: the final `requirements` bullet. Correction: retain the algebraic, encoding, zeroization, lint, and privacy proofs; add requirements proving the declaration carries `PairingConcrete::Bls12381Arkworks` rather than text, `PAIRING_CONCRETES` contains all three real members with distinct spellings and no mock member, `TARGET_GROUP_ENCODING_IDENTIFIERS` contains both real encodings with distinct spellings, every new value identifier is real and correctly spelled, the new declared-selection arm returns no configuration, the concrete has no settings module or table, and the factory uses declaration lookup and composition admission before construction; remove the trait `DECLARATION`, trait `CONCRETE`, and `concrete_name` proofs.
* `[✅]`   `pairing/bls12_381_halo2curves`
    * `[✅]`   Start anchor: the `pairing/bls12_381_halo2curves` top-level heading; end anchor: the `module` element's `Outside` bullet. Correction: replace the concrete's text name with `PairingConcrete::Bls12381Halo2curves` throughout the objective, role, and module; state that its declaration carries that typed identifier with no duplicate text name and the same real BLS12-381 curve, both-group arithmetic, EIP-2537 precompile, and `Bls12381V1` values as the arkworks member, so no new value-identifier variant is needed; give this settings-free concrete no settings module, node, or table and its real selection no settings; require declaration lookup and composition admission before construction, and describe this node as the increment adding its member variant, spelling, declared-set entry, and declared-selection builder arm.
    * `[✅]`   Start anchor: the `role` bullet that currently says `no file of the factory module changes`; end anchor: that same bullet. Correction: replace that statement with explicit increments to `factory/interface.rs` and `factory/mock.rs`, and add both files to the node's file list: add `PairingConcrete::Bls12381Halo2curves`, its `name()` spelling `bls12381_halo2curves`, and its entry in `PAIRING_CONCRETES` beside the three existing real members; add the `build_pairing_concrete` arm returning this real selection without configuration; leave `PairingCurve`, `VerifierGroupArithmetic`, `PrecompileEncoding`, `TargetGroupEncodingIdentifier`, and `TARGET_GROUP_ENCODING_IDENTIFIERS` unchanged because this member provides no new value.
    * `[✅]`   Start anchor: the `deps` bullet naming `PairingDeclaration` and `PairingConcrete`; end anchor: that same bullet. Correction: name the corrected `PairingDeclaration` with its typed `concrete` field, `PairingConcrete` with `Bls12381Halo2curves` and its `name()` mapping, `PAIRING_CONCRETES` with all four real members, the existing real BLS curve, both-group arithmetic, EIP-2537, and `Bls12381V1` values, `TARGET_GROUP_ENCODING_IDENTIFIERS` with both real encoding identifiers, and `build_pairing_concrete` with this member's no-configuration arm under the mocks feature; keep every trait and sampling-bound type and builder, without naming a settings module because this member has none.
    * `[✅]`   Start anchor: `adapters/pairing/src/bls12_381_halo2curves/interface.rs`; end anchor: the `CONCRETE` branch in `bls12_381_halo2curves/interaction.spec.md`. Correction: declare `Bls12381Halo2curvesPairing::DECLARATION` in the concrete's interface with `concrete: PairingConcrete::Bls12381Halo2curves`, `PairingCurve::Bls12381`, `VerifierGroupArithmetic::BothGroups`, `PrecompileEncoding::Eip2537`, `TargetGroupEncodingIdentifier::Bls12381V1`, and the declared versions; keep the constructor settings-free and state that no settings module exists; revise the interaction spec to read that interface-held declaration through `pairing/lookup_pairing_declaration` without an instance, remove the separate `CONCRETE` section and every trait-constant reference, and delete the `concrete_name` literal.
    * `[✅]`   Start anchor: the compile-time assertion over `Bls12381Halo2curvesPairing::DECLARATION`; end anchor: the compile-time assertion over `<Bls12381Halo2curvesPairing as IPairingAdapter>::CONCRETE`. Correction: assert the interface-held declaration's `concrete` is `PairingConcrete::Bls12381Halo2curves` and every other field is its real BLS12-381-provided value, with no text-name assertion; remove the trait `DECLARATION` and `CONCRETE` assertions because those constants no longer exist; add coverage proving `PAIRING_CONCRETES` contains all four real members, `TARGET_GROUP_ENCODING_IDENTIFIERS` contains both real encodings, the new spelling is distinct, and the new declared-selection arm returns no configuration.
    * `[✅]`   Start anchor: `construction`; end anchor: the final `bls12_381_halo2curves/mod.rs` bullet. Correction: keep `Bls12381Halo2curvesPairing::try_new` as the concrete's only producer, but have the factory obtain this member's declaration through `pairing/lookup_pairing_declaration` and complete `pairing/admit_pairing_composition` before calling it and before its correction computation; move `DECLARATION` out of `mod.rs` into the interface; remove the trait `DECLARATION` and `CONCRETE` implementations and adjust imports so the interface, not the trait, owns the declaration.
    * `[✅]`   Start anchor: `directionality`; end anchor: the bullet beginning `pairing/factory` constructs this concrete. Correction: replace `no concrete depends on it` with the actual family dependencies: `pairing/lookup_pairing_declaration` reaches this interface-held declaration through the family's private path, selection and composition admission consume the corrected member identifier and declared set, and the factory uses those admissions before constructing this member; keep the concrete independent of every other concrete, and state that its family-interface and mock increments introduce no dependency outside the pairing family.
    * `[✅]`   Start anchor: `requirements`; end anchor: the final `requirements` bullet. Correction: retain the algebraic, encoding, zeroization, lint, and privacy proofs; add requirements proving the declaration carries `PairingConcrete::Bls12381Halo2curves` rather than text, `PAIRING_CONCRETES` contains all four real members with distinct spellings and no mock member, `TARGET_GROUP_ENCODING_IDENTIFIERS` contains both real encodings with distinct spellings, the new declared-selection arm returns no configuration, the concrete has no settings module or table, and the factory uses declaration lookup and composition admission before construction; remove the trait `DECLARATION`, trait `CONCRETE`, and `concrete_name` proofs.
* `[✅]`   `pairing/lookup_pairing_declaration` — Halt: This node must be authored
* `[✅]`   `pairing/admit_pairing_selection` — Halt: This node must be authored
* `[✅]`   `pairing/admit_pairing_composition` — Halt: This node must be authored
* `[✅]`   `pairing/factory` — Halt: This node must be revised
* `[✅]`   `harness-crypto/benchmark`
    * `[✅]`   Start anchor: `context_slice`; end anchor: `Integration: public surface`. Correction: remove every `P::DECLARATION` and `P::CONCRETE` read from the context, interaction specification, carried entry, and public surface; use `params.concrete`, the already-selected typed `PairingConcrete` identifier, for the measurement's `concrete` and for the statement that the selected identifier is carried into the result, introducing no declaration-lookup dependency because the benchmark uses no other declaration field.
    * `[✅]`   Start anchor: `consume_pairing_draws_the_length_the_sampling_bound_requires`; end anchor: that block's final `Assert` line. Correction: change the contract and assertion from `P::CONCRETE` to `params.concrete`, so the test proves the measurement carries the selected typed identifier without relying on the removed trait constant; preserve the proof that both scalars are drawn at the sampling bound's required length and that no timing or mean is asserted.
    * `[✅]`   Start anchor: `requirements`; end anchor: the final `requirements` bullet. Correction: retain the existing draw, sampling, error, timing, lint, and privacy requirements; add a requirement proving each measurement carries `params.concrete`, the selected typed `PairingConcrete` identifier, with no read of `P::DECLARATION` or `P::CONCRETE` and no declaration-lookup dependency introduced merely to recover the selection the caller already holds.
* `[✅]`   `domain/deployment_identity`
* `[✅]`   `domain/suite_identifier`
* `[✅]`   `domain/parameter_set_identifier`
* `[✅]`   `domain/group_index`
* `[✅]`   `domain/piece_geometry`
* `[✅]`   `hashing/blake3_bao`
    * `[✅]`   Start anchor: the `hashing/blake3_bao` top-level heading; end anchor: the `module` element's `Outside` bullet. Correction: state that this node authors the family's typed member identifier `CommitmentConcrete`, real-only declared set `COMMITMENT_CONCRETES`, and total member spellings alongside the commitment-scheme value identifier and its total spelling and `uint8` wire code; the declaration carries typed member identity and no duplicate text name, real selections carry no settings and selection admission refuses every supplied setting, no settings module or table exists, `MockV1` is absent from the real value identifier, declaration reads use `hashing/lookup_hashing_declaration`, and composition admission precedes construction.
    * `[✅]`   Start anchor: the `CommitmentSchemeIdentifier` bullet in `factory/interface.rs`; end anchor: the `CommitmentDeclaration` bullet. Correction: keep only `Blake3BaoV1` in `CommitmentSchemeIdentifier`, removing `MockV1`, and declare its total spelling `blake3_bao_v1` and `uint8` wire code `1`; add `CommitmentConcrete` with real variant `Blake3Bao` and, under mocks, `Mock(MockICommitmentAdapterFailureMode)`, with spellings `blake3_bao` and `mock`; add real-only `COMMITMENT_CONCRETES` containing `Blake3Bao`; add `concrete: CommitmentConcrete` to `CommitmentDeclaration` without adding a text-name field.
    * `[✅]`   Start anchor: the `ICommitmentAdapter` bullet in `factory/interface.rs`; end anchor: that same bullet. Correction: remove `declaration(&self)` while preserving every commitment method; a member's declaration, including its `concrete` identifier and provided scheme values, is read through `hashing/lookup_hashing_declaration` without constructing the member, not through a second instance-based declaration surface.
    * `[✅]`   Start anchor: `adapters/hashing/src/blake3_bao/interface.rs`; end anchor: the `declaration()` branch in `blake3_bao/interaction.spec.md`. Correction: declare `Blake3BaoCommitment::DECLARATION` in the concrete's interface with `concrete: CommitmentConcrete::Blake3Bao`, `CommitmentSchemeIdentifier::Blake3BaoV1`, the declared chunk and key lengths, and the declared versions; keep the constructor settings-free and state that no settings module exists; revise the interaction spec to read that interface-held declaration through `hashing/lookup_hashing_declaration` without an instance and remove the separate `declaration()` branch.
    * `[✅]`   Start anchor: the `CommitmentDeclarationOverrides` bullet in `factory/mock.rs`; end anchor: the `MockICommitmentAdapterConstructorParamsOverrides` and `build_mock_i_commitment_adapter` bullet. Correction: add the typed `concrete` field to declaration building and invalidation, default it to `CommitmentConcrete::Mock(MockICommitmentAdapterFailureMode::NoFailure)`, and default the scheme to the real `Blake3BaoV1` value; render only canonical identifier spellings, removing `MockV1` and variant-name text; add `MOCK_I_COMMITMENT_ADAPTER_DECLARATION` with the mock member identifier and real scheme value, with lookup substituting the selected mock identifier; remove the inherent mock declaration and instance method; add fieldless `CommitmentConcreteOverrides` and `build_commitment_concrete` over `COMMITMENT_CONCRETES`, whose real arm returns the selection without configuration.
    * `[✅]`   Start anchor: `declaration_returns_the_blake3_bao_declaration`; end anchor: that block's final `Assert` line. Correction: replace the instance-based declaration test with a declaration-lookup test asserting `concrete: CommitmentConcrete::Blake3Bao`, the real `Blake3BaoV1` scheme, declared lengths, and versions, with no adapter constructed; add coverage proving `COMMITMENT_CONCRETES` contains only `Blake3Bao`, member spellings are distinct, `Blake3BaoV1` has spelling `blake3_bao_v1` and wire code `1`, no mock-only scheme variant exists, and the declared-selection builder returns no settings.
    * `[✅]`   Start anchor: `construction`; end anchor: the final `blake3_bao/mod.rs` bullet. Correction: keep `Blake3BaoCommitment::try_new` as the concrete's only producer, but have the factory obtain this member's declaration through `hashing/lookup_hashing_declaration` and complete `hashing/admit_hashing_composition` before calling it; move `DECLARATION` out of `mod.rs` into the interface; remove the instance `declaration()` implementation and adjust imports so the interface owns the declaration and the trait carries only commitment operations.
    * `[✅]`   Start anchor: `directionality`; end anchor: that element's only bullet. Correction: preserve the existing runtime and vendor boundaries while stating the actual family dependencies: `hashing/lookup_hashing_declaration` reaches this interface-held declaration through the family's private path, selection and composition admission consume `CommitmentConcrete`, `COMMITMENT_CONCRETES`, and the real scheme identifier, and the factory completes composition admission before constructing this member; no consumer depends on the concrete or reads an instance declaration.
    * `[✅]`   Start anchor: `requirements`; end anchor: the final `requirements` bullet. Correction: retain the root, vector, verification, challenge, integration, lint, and privacy proofs; add requirements proving the declaration carries `CommitmentConcrete::Blake3Bao` and the real `Blake3BaoV1` scheme, `COMMITMENT_CONCRETES` contains only real members with distinct spellings, `Blake3BaoV1` has spelling `blake3_bao_v1` and wire code `1`, `MockV1` does not exist, real selections carry no settings and selection admission refuses every supplied setting, the declared-selection builder returns no configuration, and declaration lookup and composition admission precede construction; remove every instance `declaration()` proof.
* `[✅]`   `hashing/lookup_hashing_declaration` — Halt: This node must be authored
* `[✅]`   `hashing/admit_hashing_selection` — Halt: This node must be authored
* `[✅]`   `hashing/admit_hashing_composition` — Halt: This node must be authored
* `[ ]`   `hashing/factory` — Halt: This node must be revised
* `[ ]`   `encoding/canonical_field_reader`
* `[ ]`   `encoding/abi`
* `[ ]`   `encoding/lookup_encoding_declaration` — Halt: This node must be authored
* `[ ]`   `encoding/admit_encoding_selection` — Halt: This node must be authored
* `[ ]`   `encoding/factory` — Halt: This node must be revised
* `[ ]`   `kdf/blake3_keyed`
* `[ ]`   `kdf/lookup_kdf_declaration` — Halt: This node must be authored
* `[ ]`   `kdf/admit_kdf_selection` — Halt: This node must be authored
* `[ ]`   `kdf/factory` — Halt: This node must be revised
* `[ ]`   `hash-to-scalar/keccak256`
* `[ ]`   `hash-to-scalar/lookup_hash_to_scalar_declaration` — Halt: This node must be authored
* `[ ]`   `hash-to-scalar/admit_hash_to_scalar_selection` — Halt: This node must be authored
* `[ ]`   `hash-to-scalar/factory` — Halt: This node must be revised

### Signature services (protocol core; runs beside the harness from `encoding/factory`)

* `[ ]`   `signature/ed25519`
* `[ ]`   `signature/secp256k1`
* `[ ]`   `signature/ml_dsa_65`
* `[ ]`   `signature/lookup_signature_declaration` — Halt: This node must be authored
* `[ ]`   `signature/admit_signature_selection` — Halt: This node must be authored
* `[ ]`   `signature/admit_signature_composition` — Halt: This node must be authored
* `[ ]`   `signature/factory` — Halt: This node must be revised

### Credential KEM

* `[ ]`   `kem/bb1_depth_one`
* `[ ]`   `kem/lookup_kem_declaration` — Halt: This node must be authored
* `[ ]`   `kem/admit_kem_selection` — Halt: This node must be authored
* `[ ]`   `kem/admit_kem_composition` — Halt: This node must be authored
* `[ ]`   `kem/factory` — Halt: This node must be revised
* `[ ]`   `workflows/sidecar/derivation_context`
* `[ ]`   `workflows/sidecar/wrap`
* `[ ]`   `workflows/sidecar/unwrap`

### Envelope

* `[ ]`   `envelope/possession_statement`
* `[ ]`   `envelope/pairing_elgamal`
* `[ ]`   `envelope/lookup_envelope_declaration` — Halt: This node must be authored
* `[ ]`   `envelope/admit_envelope_selection` — Halt: This node must be authored
* `[ ]`   `envelope/factory` — Halt: This node must be revised

### Delivery proof and on-chain verifier

*Chain-forms family — precedes every proof ticket (requires `signature/factory`):*

* `[ ]`   `chain-forms/evm`
* `[ ]`   `chain-forms/lookup_chain_declaration` — Halt: This node must be revised
* `[ ]`   `chain-forms/admit_chain_selection` — Halt: This node must be revised
* `[ ]`   `chain-forms/admit_chain_composition` — Halt: This node must be revised
* `[ ]`   `chain-forms/factory` — Halt: This node must be revised

*Proof family:*

* `[ ]`   `proof/mint_statement`
* `[ ]`   `proof/transfer_statement`
* `[ ]`   `proof/schnorr_fs/challenge`
* `[ ]`   `proof/schnorr_fs`
* `[ ]`   `proof/lookup_proof_declaration` — Halt: This node must be authored
* `[ ]`   `proof/admit_proof_selection` — Halt: This node must be authored
* `[ ]`   `proof/admit_proof_composition` — Halt: This node must be authored
* `[ ]`   `proof/factory` — Halt: This node must be revised

*Delivery functions:*

* `[ ]`   `harness-crypto/delivery/mint`
* `[ ]`   `harness-crypto/delivery/transfer`

*Generate family:*

* `[ ]`   `harness-crypto/generate/solidity/render`
* `[ ]`   `harness-crypto/generate/solidity/constants`
* `[ ]`   `harness-crypto/generate/solidity/group_vectors`
* `[ ]`   `harness-crypto/generate/solidity/rejection_vectors`
* `[ ]`   `harness-crypto/generate/solidity/mapping_vectors`
* `[ ]`   `harness-crypto/generate/solidity/possession_vectors`
* `[ ]`   `harness-crypto/generate/solidity/challenge_vectors`
* `[ ]`   `harness-crypto/generate/solidity/proof_vectors`
* `[ ]`   `harness-crypto/generate/solidity/settings`
* `[ ]`   `harness-crypto/generate/solidity`
* `[ ]`   `harness-crypto/generate/lookup_generate_declaration`
* `[ ]`   `harness-crypto/generate/admit_generate_selection`
* `[ ]`   `harness-crypto/generate/admit_generate_composition`
* `[ ]`   `harness-crypto/generate/factory`

*Path to `chain/factory`:*

* `[ ]`   `contracts/evm/ParameterSets`
* `[ ]`   `domain/handshake_key_commitment`
* `[ ]`   `chain/base/settings`
* `[ ]`   `chain/base`
* `[ ]`   `chain/lookup_chain_declaration` — Halt: This node must be authored
* `[ ]`   `chain/admit_chain_selection` — Halt: This node must be authored
* `[ ]`   `chain/admit_chain_composition` — Halt: This node must be authored
* `[ ]`   `chain/factory` — Halt: This node must be revised

*Verifier family:*

* `[ ]`   `harness-crypto/verifier/evm`
* `[ ]`   `harness-crypto/verifier/lookup_verifier_declaration`
* `[ ]`   `harness-crypto/verifier/admit_verifier_selection`
* `[ ]`   `harness-crypto/verifier/admit_verifier_composition`
* `[ ]`   `harness-crypto/verifier/factory`

*Measurement and report:*

* `[ ]`   `harness-crypto/measure`
* `[ ]`   `harness-crypto/report`

*Configuration:*

* `[ ]`   `harness-crypto/config/layers`
* `[ ]`   `harness-crypto/config/admit_selections`
* `[ ]`   `harness-crypto/config/admit_statements`
* `[ ]`   `harness-crypto/config/admit_generate`
* `[ ]`   `harness-crypto/config/admit_output`
* `[ ]`   `harness-crypto/config`

*Run, consumers innermost first:*

* `[ ]`   `harness-crypto/run/admit_contexts`
* `[ ]`   `harness-crypto/run/generate_consumer`
* `[ ]`   `harness-crypto/run/verifier_consumer`
* `[ ]`   `harness-crypto/run/delivery_proof_consumer`
* `[ ]`   `harness-crypto/run/key_agreement_consumer`
* `[ ]`   `harness-crypto/run/kem_consumer`
* `[ ]`   `harness-crypto/run/chain_consumer`
* `[ ]`   `harness-crypto/run/chain_forms_consumer`
* `[ ]`   `harness-crypto/run/encoding_consumer`
* `[ ]`   `harness-crypto/run/pairing_consumer`
* `[ ]`   `harness-crypto/run`
* `[ ]`   `harness-crypto/output`
* `[ ]`   `harness-crypto/main`

*Contract suite:*

* `[ ]`   `contracts/evm/PairingLib`
* `[ ]`   `contracts/evm/DeliveryVerifier`
* `[ ]`   `contracts/evm/IdentityAccount`
* `[ ]`   `contracts/evm/Binding`
* `[ ]`   `contracts/evm/Registry`
* `[ ]`   `contracts/evm/deploy` 

### Swarm transport and seed host

* `[ ]`   `transport/rqbit/settings` — Halt: This node must be authored
* `[ ]`   `transport/rqbit` — Halt: This node must be revised
* `[ ]`   `transport/lookup_transport_declaration` — Halt: This node must be authored
* `[ ]`   `transport/admit_transport_selection` — Halt: This node must be authored
* `[ ]`   `transport/factory` — Halt: This node must be revised
* `[ ]`   `discovery/admit_peers`
* `[ ]`   `discovery/local/settings` — Halt: This node must be authored
* `[ ]`   `discovery/local` — Halt: This node must be revised
* `[ ]`   `discovery/locator_of`
* `[ ]`   `discovery/dht/settings` — Halt: This node must be authored
* `[ ]`   `discovery/dht` — Halt: This node must be revised
* `[ ]`   `discovery/pex/settings` — Halt: This node must be authored
* `[ ]`   `discovery/pex` — Halt: This node must be revised
* `[ ]`   `discovery/tracker/settings` — Halt: This node must be authored
* `[ ]`   `discovery/tracker` — Halt: This node must be revised
* `[ ]`   `discovery/aggregate`
* `[ ]`   `discovery/lookup_discovery_declaration` — Halt: This node must be authored
* `[ ]`   `discovery/admit_discovery_selection` — Halt: This node must be authored
* `[ ]`   `discovery/admit_discovery_composition` — Halt: This node must be authored
* `[ ]`   `discovery/factory` — Halt: This node must be revised
* `[ ]`   `storage/redb`
* `[ ]`   `storage/lookup_storage_declaration` — Halt: This node must be authored
* `[ ]`   `storage/admit_storage_selection` — Halt: This node must be authored
* `[ ]`   `storage/factory` — Halt: This node must be revised
* `[ ]`   `seed-host/owned/settings` — Halt: This node must be authored
* `[ ]`   `seed-host/owned` — Halt: This node must be revised
* `[ ]`   `seed-host/rqbit/settings` — Halt: This node must be authored
* `[ ]`   `seed-host/rqbit` — Halt: This node must be revised
* `[ ]`   `seed-host/lookup_seed_host_declaration` — Halt: This node must be authored
* `[ ]`   `seed-host/admit_seed_host_selection` — Halt: This node must be authored
* `[ ]`   `seed-host/factory` — Halt: This node must be revised

## Completion check

Each proof is a search of the specification layer, the planning layer, and the workplans, excluding this document, or a comparison between them. The scope is complete when every proof below passes.

Each proof entry is appended at the same time as the correction item it proves.

