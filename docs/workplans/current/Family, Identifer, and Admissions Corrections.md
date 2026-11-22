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
- Input text is admitted by the identifier's owner against the spelling of each declared real member, with unknown text refused by a typed error carrying that text. A family's selection admission receives the admitted identifier and the untrusted settings document. For a member with a settings module it narrows that document through the settings module's guard, which refuses a key naming no field of the settings type, invokes the settings admission, which refuses a value outside its declared bounds and takes the declared default where the document supplies none, and returns the selection carrying the admitted settings; for a member with no settings module it refuses a document that supplies any setting, with the family's own error naming the member, and returns the selection naming the member. An absent member list is the declared set, and every callee error is carried unchanged.
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
- New or rewritten bullets in a `[ ]` node take `[ ]`. Bullets in a `[✅]` node keep `[✅]`.
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

* `[ ]`   `workspace/cargo`
* `[ ]`   `domain/secret`
* `[ ]`   `random/os`
* `[ ]`   `random/lookup_random_declaration` — Halt: This node must be authored
* `[ ]`   `random/admit_random_selection` — Halt: This node must be authored
* `[ ]`   `random/admit_random_composition` — Halt: This node must be authored
* `[ ]`   `random/factory` — Halt: This node must be revised
* `[ ]`   `workspace/ci`

### Pairing adapters and key derivation (harness grouping)

* `[ ]`   `pairing/bn254_arkworks`
* `[ ]`   `pairing/bn254_halo2curves`
* `[ ]`   `pairing/bls12_381_arkworks`
* `[ ]`   `pairing/bls12_381_halo2curves`
* `[ ]`   `pairing/lookup_pairing_declaration` — Halt: This node must be authored
* `[ ]`   `pairing/admit_pairing_selection` — Halt: This node must be authored
* `[ ]`   `pairing/admit_pairing_composition` — Halt: This node must be authored
* `[ ]`   `pairing/factory` — Halt: This node must be revised
* `[ ]`   `harness-crypto/benchmark`
* `[ ]`   `domain/deployment_identity`
* `[ ]`   `domain/suite_identifier`
* `[ ]`   `domain/parameter_set_identifier`
* `[ ]`   `domain/group_index`
* `[ ]`   `domain/piece_geometry`
* `[ ]`   `hashing/blake3_bao`
* `[ ]`   `hashing/lookup_hashing_declaration` — Halt: This node must be authored
* `[ ]`   `hashing/admit_hashing_selection` — Halt: This node must be authored
* `[ ]`   `hashing/admit_hashing_composition` — Halt: This node must be authored
* `[ ]`   `hashing/factory` — Halt: This node must be revised
* `[ ]`   `encoding/canonical_field_reader`
* `[ ]`   `encoding/abi`
* `[ ]`   `encoding/lookup_encoding_declaration` — Halt: This node must be authored
* `[ ]`   `encoding/admit_encoding_selection` — Halt: This node must be authored
* `[ ]`   `encoding/admit_encoding_composition` — Halt: This node must be authored
* `[ ]`   `encoding/factory` — Halt: This node must be revised
* `[ ]`   `kdf/blake3_keyed`
* `[ ]`   `kdf/lookup_kdf_declaration` — Halt: This node must be authored
* `[ ]`   `kdf/admit_kdf_selection` — Halt: This node must be authored
* `[ ]`   `kdf/admit_kdf_composition` — Halt: This node must be authored
* `[ ]`   `kdf/factory` — Halt: This node must be revised
* `[ ]`   `hash-to-scalar/keccak256`
* `[ ]`   `hash-to-scalar/lookup_hash_to_scalar_declaration` — Halt: This node must be authored
* `[ ]`   `hash-to-scalar/admit_hash_to_scalar_selection` — Halt: This node must be authored
* `[ ]`   `hash-to-scalar/admit_hash_to_scalar_composition` — Halt: This node must be authored
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
* `[ ]`   `envelope/admit_envelope_composition` — Halt: This node must be authored
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
* `[ ]`   `transport/admit_transport_composition` — Halt: This node must be authored
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
* `[ ]`   `storage/admit_storage_composition` — Halt: This node must be authored
* `[ ]`   `storage/factory` — Halt: This node must be revised
* `[ ]`   `seed-host/owned/settings` — Halt: This node must be authored
* `[ ]`   `seed-host/owned` — Halt: This node must be revised
* `[ ]`   `seed-host/rqbit/settings` — Halt: This node must be authored
* `[ ]`   `seed-host/rqbit` — Halt: This node must be revised
* `[ ]`   `seed-host/lookup_seed_host_declaration` — Halt: This node must be authored
* `[ ]`   `seed-host/admit_seed_host_selection` — Halt: This node must be authored
* `[ ]`   `seed-host/admit_seed_host_composition` — Halt: This node must be authored
* `[ ]`   `seed-host/factory` — Halt: This node must be revised

## Completion check

Each proof is a search of the specification layer, the planning layer, and the workplans, excluding this document, or a comparison between them. The scope is complete when every proof below passes.

Each proof entry is appended at the same time as the correction item it proves.

