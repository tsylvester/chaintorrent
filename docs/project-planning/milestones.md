<!-- Template: parenthesis_milestone_schema.md -->
# Index

The milestones for the ChainTorrent MVP: the middle view between the plan and the workplan's nodes, spanning the empty repository to the delivered MVP. Milestones are addressed by their dependency role and never by ordinal; each names what precedes it and what it unblocks, so insertion and reordering leave every other milestone's text valid.

- Executive Summary
- Pipeline Context
- Selection Criteria
- Shared Infrastructure
- Milestones, grouped by delivery role: foundation; cryptographic validation harness; protocol core and contract suite; local daemon and package serving; onboarding shells and services; acceptance and release
- Iteration Semantics
- Features Context
- Feasibility Insights
- Non-Functional Alignment
- Architecture Summary, Services, Components, Dependency Resolution, Component Details, Integration Requirements
- Migration Context

Sources: [product requirements](product-requirements.md), [technical requirements](technical-requirements.md), [system architecture](system-architecture.md), [dependency map](dependency-map.md), [tech stack](tech-stack.md), [feature spec](feature-spec.md), [success metrics](success-metrics.md), [risk register](risk-register.md), and the specifications under [docs/research](../research/) and the [workplan](../workplans/current/ChainTorrent%20MVP.md).

# Executive Summary

The milestones, in groupings by delivery role, carry the work from an empty repository to a released MVP. The foundation grouping establishes the workspace and its lint table, the secret type, the randomness family, and the build on Windows, macOS, and Linux.
The cryptographic validation harness grouping builds the domain's identifier modules, the hashing family with its root type, the encoding family whose first description is the root type's, the derivation context in the sidecar layer, and the pairing, key-derivation, hash-to-scalar, credential KEM, envelope, and delivery proof families, each from its first ticket, which carries the family's interface, to its factory, which follows every concrete, together with the chain family's EVM forms concrete and its family-owned forms function,
the mint and transfer delivery functions every proof-producing ticket consumes, the on-chain verifier, the parameter-set contract, which creates the suite's toolchain, the Base concrete, which authors the ABI descriptions of the contracts it binds, and the chain family's factory with its selection admission, declaration lookup, and composition admission, which the harness's verifier, configuration, and run consume,
the harness's own generate and verifier families, the harness's typed configuration, run, output, and binary, and the identity account, binding, and registry contracts, which read the generated constants, measures them on Base Sepolia against the configured piece-group size and curve defaults, records any adjustment the data supports, and yields the project's throughput calibration.
The protocol core grouping builds the signature, transport, discovery, storage, and seed-host families at ticket resolution beside the harness, without any chain, with `librqbit` as the transport concrete, and then the domain model, the cipher family with the sidecar layer, the remainder of the EVM contract suite as the chain family's on-chain concrete with its on-chain adapter registry, and the chain and submission families' remaining tickets after the registry and entitlement contracts.
The daemon grouping builds durable jobs and configuration, identity and custody once the relayer's submission factory exists, the plaintext CAS and package host, the First Finder engine, and credential delivery and per-attempt authorization, then the resolution orchestrator over every source, then the daemon composition, the composition resolver, the settings service, and the daemon and installer binaries, once every module they compose exists, the installer's, the publisher path's, and the claim verifier's among them, and then reaches the demonstrable milestone at which an ordinary `npm install` is served at the registry's speed, registers and prefetches, a second identity resolves against the swarm with the registry down, and the north star, first-run independence, and install wall-clock are observed. The shells and services grouping builds the relayer ahead of the identity milestone; the installation coordinator's families and workflow on Windows, macOS, and Linux, the publisher path, and the claim verifier ahead of the daemon composition; and the control surfaces, the site with its WebAssembly demonstration, observability reconciliation, and the demonstration harness after it. The acceptance and release grouping runs the transaction flow proof on the Base mainnet pilot, closes external review and legal prerequisites, runs every acceptance scenario on clean machines, records the dogfood baseline, and releases.

Each milestone states its purpose, scope by subsystem and ticket, entry conditions, exit proof by requirement and scenario identifier, deliverables, owner role, and what it unblocks. Resolution follows the dependency map's decay: the foundation and harness groupings, the signature and swarm milestones, and the tickets on the path to `chain/factory` are at ticket resolution; the remaining milestones are re-mapped to tickets as the harness closes and outward from there. The stop criteria from the product requirements sit at the milestones where they can first be evaluated.

# Pipeline Context

This document sits between the product and technical requirements above it and the workplan's nodes below it. The [technical requirements](technical-requirements.md) fix what the system must contain; the [dependency map](dependency-map.md) fixes what depends on what and holds the harness at ticket resolution; this document segments that into milestones with entry and exit conditions; the workplan's Work Breakdown Structure then holds one node per source file, authored through the repository's ordinary path from the tickets a milestone names. The master plan, which the pipeline places before this document, summarizes these milestones into the implementation view; it can be produced from this document without loss.

Two repository rules govern everything below. Nodes and groupings are addressed relationally, never numbered. Each node is one source file with its full support system, authored test-first in the fixed element order, one file per turn, with a commit only at the end of a chain that can be integration-tested.

# Selection Criteria

A milestone is drawn where all of the following hold.

- **It ends at a provable boundary.** Its exit is a set of requirement rows and, where one exists, an acceptance scenario, proven through the boundary the requirement names, never by a unit test where the proof crosses a process, adapter, storage, network, custody, or chain boundary.
- **Its dependencies are complete before it starts.** Entry conditions name the milestones that must have closed; nothing inside a milestone waits on something outside it.
- **Its entry conditions are code dependencies.** A milestone depends on another only when a file it authors needs a file the other authors in order to be written: a type, a trait, a function, or an interface it imports, or a configuration file it is built under. What a file needs only in order to run, output a program writes, a deployed contract, a measured value, is a runtime input; the dependency map's Runtime sequence holds the harness's, a milestone whose exit runs another milestone's output states it as a runtime input, and no runtime input is an entry condition.
- **It is the smallest span that yields something the next milestone consumes.** A milestone that produces nothing consumable is merged into its consumer; one that produces two independent things is split.
- **It carries a commit when it authors files.** The last node in a milestone's chain holds the integration test and the commit, per the workplan-structure rule that neither is ever a node of its own; a milestone that authors no file, the harness measurement, the demonstrable milestone, and the acceptance and release milestones, closes on its exit.
- **Where a stop criterion can first be evaluated, the milestone names it.** The post-harness re-map, the acceptance run, and review closure each carry one.
- **It has one owner by role.** Roles are those the product requirements and success metrics use.
- **It begins each family it introduces at that family's first ticket and closes the family at its factory.** Where the milestone is at ticket resolution, the first ticket, the family's first concrete or a family-owned function preceding it, authors the family's generic interface, capability declaration, and mock as its producers and creates the crate; each further concrete follows carrying its own settings-document type, its guard, and the admission of its own settings, and each family-owned function — declaration lookup, selection admission, and composition admission — follows ahead of the factory; the factory ticket, which constructs only settings-carrying selections the family's admission functions have returned, follows every concrete and every family-owned function, carries the family's integration test across factory and concretes, and carries the milestone's commit where the family closes the milestone's chain; the factory ticket is authored once, after the family's last concrete, and precedes every consumer, each consumer depending on the specific admission surface it uses — selection admission, declaration lookup, composition admission — or the factory, rather than on the factory alone. No ticket consumes a concrete, and the declarations exist from the family's first ticket. Where the milestone is not yet at ticket resolution, this rule holds at the milestone's existing resolution only — the family begins ahead of its consumers and admission precedes construction — without inventing ticket detail.

# Shared Infrastructure

Built once in the foundation grouping and consumed by every later milestone.

| Infrastructure | Provides | Built in |
| --- | --- | --- |
| Cargo workspace with rings as crates | A domain crate, a workflows crate, one crate per adapter family holding its `factory` module and its private concretes, apps, each created by the node of the first module that lives in it; glob members; a lint table forbidding `unsafe_code` and denying `unwrap_used`, `expect_used`, `panic`, and `as_conversions` in production code; a ring violation is a compile error | Workspace and discipline bootstrap for the manifest; each crate at its first module |
| Continuous integration on clean ephemeral runners for Windows, macOS, and Linux | `cargo check`, `cargo clippy`, `cargo fmt --check`, the tests, `cargo-audit`, `cargo-deny` on every push in the Rust workflow definition; `forge build`, `forge fmt --check`, and `forge test`, the TypeScript linter, `cargo-fuzz`, and end-to-end scenarios on clean runners each in a workflow definition of its own, authored once and complete by the ticket that first needs it, so no ticket amends a definition another ticket created | Workspace and discipline bootstrap for the Rust definition; `contracts/evm/ParameterSets`, the contract suite's first source file, for the Solidity definition; the first shell or webview ticket for the TypeScript definition; the first fuzz target and the first end-to-end scenario for theirs |
| Encoding family | `encoding/canonical_field_reader`, creating the crate with the canonical field types and the reader every field list is checked through; `encoding/abi`, implementing `IEncoderAdapter` and `IDecoderAdapter` through `alloy`'s sol types, which the EVM suite reproduces natively, and authoring them under one versioned encoding identifier with the declaration, proven over the root type's description and the byte-string and width vectors, for everything hashed, signed, stored, or framed over IPC, with its settings-document guard and settings admission; the family's member and encoding identifier types and its declaration lookup, selection admission, and composition admission authored ahead of `encoding/factory`, which follows them | Pairing adapters and key derivation, after the hashing family |
| Randomness family | `random/os` over the operating system's generator, authoring the interface that fills bytes, the declaration, and the family's mock, with its own settings admission; the family's declaration lookup, selection admission, and composition admission; `random/factory` following them with the family's integration test and the grouping's commit | Workspace and discipline bootstrap |
| Secret type | A `domain` type that cannot be formatted or serialized, exposes the wrapped value only through an explicit accessor, and zeroizes on drop through `zeroize`, a domain crate dependency | Workspace and discipline bootstrap |
| Telemetry family and tracing with correlation | `telemetry/local_metrics`, authoring the exporter interface and its own settings admission; `telemetry/tracing`, per-request correlation identifiers across every subsystem and its own settings admission; `telemetry/opentelemetry`, the OpenTelemetry exporter concrete and its own settings admission; the family's declaration lookup, selection admission, and composition admission; `telemetry/factory`, following the concretes and the three functions | Durable jobs and configuration |
| Storage engine family | `storage/redb`, authoring the key-value store interface with tables, transactions, and checkpoints, and its own settings admission; the family's declaration lookup, selection admission, and composition admission; `storage/factory` following them; consumed through its admission surfaces and factory by the job engine, the configuration registry, and every index | Swarm transport and seed host, at its first consumer |
| Local chain and test chain | Anvil in the developer loop; Base Sepolia for deployment; Foundry scripts under `contracts/evm` emitting each deployed address as a deployment record keyed by its suite identifier and pairing-owned curve identifier, a runtime output a later run reads as a configuration layer over the shipped defaults | Delivery proof and on-chain verifier |
| Generated verifier constants and vectors | The constants library and the vector-record library under the generated directory and, in each cell directory beneath it, one vector file per vector kind the generate member declares under its vector-kind identifier, produced from the Rust reference by `harness-crypto/generate/solidity`, the generate member named for the source-language identifier it emits, admitted through the generate family's selection and composition admission before it writes, and the functions it owns, each vector file one ABI-encoded, language-neutral record per line rendering the record's canonical wire description at that file boundary, holding its kind's configured count, enumerated set, or configured inputs, every file and library name the configuration's, written under the generated directory `contracts/evm/foundry.toml` grants, which the configuration's shipped default names, when the harness binary's run calls the generator, and committed, since the Solidity continuous-integration definition runs only Foundry; Foundry tests enumerate the cell directories, join each kind's file name, which the vector-record library carries, to the cell directory they read, and decode each line with that library's function for the kind | Delivery proof and on-chain verifier |
| Harness configuration | `harness-crypto/config`, the typed configuration the harness owns with a shipped default for every value it passes as params or payload — the matrix's lists excepted — the member and value identifier per family, the generate member named for the source-language identifier it emits, the matrix's lists of pairing members, target-group encoding identifiers, and encodings, each the family's real declared set where the configuration carries none, the forms identifier, the identity scope, the delivery-statement version, the chain selection, its member under its own settings table, the statements' context values, the identities admitted under the identity scope, the iteration counts, the per-kind settings of each declared vector kind, its file, struct, decode function, and path constant names and, for a drawn kind, its count, the generated libraries' file and library names, the output paths, the Foundry project root and the generated directory relative to it, the directory `contracts/evm/foundry.toml` grants, the Solidity compiler version and license identifier the generated libraries open with, one verifier selection and the deployment records, each keyed by its contract suite's declared suite identifier and the pairing-owned curve identifier, the measurement's settings, the report's destinations, and the output destinations the run's outcome is written to, standard output, a file, or both, overridable from a configuration file and from command-line flags treated as untrusted documents each identifier's owner admits, and authored after every ticket whose values it carries, the verifier, measurement, and report tickets among them; `harness-crypto/run` composes every family from it over the consumers it owns, each consumer using the admission surface it needs — selection admission, declaration lookup, composition admission, or the factory — completing composition admission before files, deployment calls, measurements, or output, admitting each configured context into the resolved forms once through `harness-crypto/run/admit_contexts`, `harness-crypto/output` writes the run's outcome, success or refusal rendered to canonical spellings or wire codes only at that boundary, to its destinations, and the binary calls the three in turn | Delivery proof and on-chain verifier |
| Code signing | Apple, Authenticode, and Sigstore identities enrolled during the harness grouping so they exist before the onboarding grouping | Workspace and discipline bootstrap, enrollment started; installation coordinator, first use |
| Demonstration harness | Controlled participants, wallets, chain state, and fault injection; load-bearing under the completion boundary | Its own milestone in the shells and services grouping |
| Metrics store and release-evidence format | The RO-03 record and the harness output document, every closed-set field carrying its owner-typed identifier and rendered to canonical spellings or wire codes only at the report boundary | Durable jobs and configuration; harness measurement and report |

# Milestones

Within each grouping, milestones are listed in dependency order; where two are independent, the text says so.

## Foundation grouping

### Workspace and discipline bootstrap

**Purpose.** Turn the empty repository into a workspace where a node can be authored and proven on Windows, macOS, and Linux.

**Scope.** The tickets `workspace/cargo`, `domain/secret`, `random/os`, `random/os/settings`, `random/lookup_random_declaration`, `random/admit_random_selection`, `random/admit_random_composition`, `random/factory`, and `workspace/ci`: the virtual workspace manifest with the glob members `crates/*`, `adapters/*`, and `apps/*`, the lint table, and the `[patch.crates-io]` overlay pointing `librqbit` at the project's overlay branch `chaintorrent-overlay` in `https://github.com/tsylvester/rqbit`, which exists at a tagged upstream release before the bootstrap as external setup; `rust-toolchain.toml`, `deny.toml` with the license allowlist and that repository as its allowed git source, and `.gitignore`; the `domain` crate, created by its secret module; the `adapters/random` crate, created by the operating-system concrete, which authors the byte-filling interface, the declaration, the family's mock, and the member identifier with its total spellings over the family's real source set as its producers, `random/os/settings` carrying the concrete's settings-document type, its guard, and the admission of its own settings, `random/lookup_random_declaration` reading declarations without constructing the concrete, `random/admit_random_selection` returning the settings-carrying selection, and `random/admit_random_composition` comparing selected requirements to declarations, the three family-owned functions authored between the concrete and `random/factory`, the factory following them, calling composition admission before it constructs anything, and carrying the family's integration test across factory and concrete; no encoding family here, since nothing encodable exists yet; no crate created ahead of the module that first lives in it, each configuration file authored once and complete by the ticket that creates it, and each crate's manifest and barrel receiving one entry per module from that module's ticket; the element mapping applied as one module directory per function, one adapter per file, and a family crate's `factory` module as its factory; the GitHub Actions matrix over Windows, macOS, and Linux running the `cargo` checks, the tests, and the audits; Apple, Microsoft, and Sigstore signing enrollment started. `random/factory` carries the milestone's integration test and commit.

**Entry.** None.

**Exit.** The workspace and every crate its tickets create build and pass checks on Windows, macOS, and Linux; the lint table rejects the denied constructs in production code; `cargo-deny` enforces the allowlist; the secret type is proven at compile time to implement no formatting or serialization trait; each family crate exposes its family-owned functions and factory's surface and no concrete; the operating-system concrete's repeated draws of a fixed width are pairwise distinct across a fixed count and a filled buffer of a fixed length holds more than one distinct byte value, the source alone being proven here, with CR-05's per-bootstrap independence proven at the First Finder engine and its reproducible publisher derivation at the explicit publisher path. Requirement rows: CR-05 for the randomness source, CR-07 for the secret type, XA-07 for the facilities, NF-M07 as the authoring discipline.

**Deliverables.** The workspace; the CI definition; the domain and random crates, the random family's member identifier, its settings admission, and its three family-owned functions among them; the signing enrollment requests filed.

**Owner.** Project lead with the platform implementer.

**Unblocks.** Everything.

## Cryptographic validation harness grouping

Mapped at ticket resolution in the dependency map. The independent starting points are `pairing/bn254_arkworks`, which carries the pairing interface with its sampling bound, and the domain's identifier modules, with the hashing family beside them; the encoding family follows the hashing family, the derivation context in the sidecar layer follows the hashing factory, the identifiers, and the encoding factory, and `kdf/blake3_keyed`, the first ticket that encodes a context, follows the encoding factory; every other ticket consumes one of them, and each family's factory follows every concrete of the family. The harness constructs every cell of the selected pairing concretes, target-group encoding identifiers, and encodings through the factories, so an invalid combination is refused here as the daemon's resolver will refuse it. External cryptographic review starts on the composition claims when this grouping starts.

### Pairing adapters and key derivation

**Purpose.** The group arithmetic and derivation every cryptographic ticket calls, on both curves, with encodings that match the precompiles, behind families whose declarations and admission functions precede their factories from the first ticket.

**Scope.** `pairing/bn254_arkworks`, authoring `IPairingAdapter`, the reference trait `IPairingReference`, beside the generic interface, with the scalar field's order and the precompile encoding of the defined point outside each source group's prime-order subgroup, the group-element and scalar associated types with the sampling bound on the scalar type, sample from uniform bytes, the declaration of the concrete's member identifier, of the curve identifier, of second-group arithmetic at the verifier, of the precompile-encoding identifiers, and of the target-group encoding identifier, and the family's mock as its producers, owning its scalar and group-element types over its library's elements behind fallible constructors, implementing the sampling bound on its scalar type, proving the in-range draw, carrying its settings-document type, its guard, and the admission of its own settings, and creating the crate;
`pairing/bn254_halo2curves`, `pairing/bls12_381_arkworks`, and `pairing/bls12_381_halo2curves`, one concrete per curve per library, each naming only its own library, each owning its scalar and group-element types over its library's elements, each carrying its settings-document type, its guard, and its settings admission, each implementing and proving the sampling bound on its scalar type, and each proven by executing the family's target-group encoding identifier's definition in its tests, the BLS12-381 concretes also against the CFRG draft's published value;
`pairing/lookup_pairing_declaration`, `pairing/admit_pairing_selection`, and `pairing/admit_pairing_composition`, the family's declaration lookup, selection admission, and composition admission, each a ticket preceding `pairing/factory`; `pairing/factory`, constructing only the admitted selection, with composition admission returning the family's `Incompatible` where a selection requires a target-group encoding identifier no composed declaration provides;
`harness-crypto/benchmark`, which admits every pairing member through the family's selection admission and has composition admission resolve it before the factory constructs it, measures scalar multiplication, multi-scalar multiplication, and pairing on each, and records each member's measurements, with every concrete remaining beneath the factory, creating the `apps/harness-crypto` crate;
the domain's identifier modules, `domain/deployment_identity`, `domain/suite_identifier`, `domain/parameter_set_identifier`, `domain/group_index`, and `domain/piece_geometry`, each with its fallible `try_new` constructor enforcing its invariants;
`hashing/blake3_bao`, implementing root and outboard construction, streaming and random-access verification, and random challenge and response as one adapter with keyed mode for the keyed plaintext-root disclosure mode, and authoring the commitment interface, the root, outboard, path, and chunk types, the root type being the object's BLAKE3/Bao root and the asset's identity with its fallible constructor refusing the all-zero value, the commitment-scheme identifier the hash-card carries, the commitment-scheme and chunk-granularity declaration, and the family's mock as its producers, carrying its settings-document type, its guard, and its settings admission, every operation over bytes, with no digest operation and no identity-hash identifier, depending on no encoding;
`hashing/lookup_hashing_declaration`, `hashing/admit_hashing_selection`, and `hashing/admit_hashing_composition` preceding `hashing/factory`, which constructs only the admitted selection, with the family admitting a member by the commitment-scheme identifier alone;
`encoding/canonical_field_reader`, the encoding family's first ticket, creating the `adapters/encoding` crate and authoring the canonical field-kind and type-name identifier types with total spellings and declared wire codes, the eight canonical field kinds and their values, the byte-string, fixed twenty-byte, and 256-bit unsigned kinds among them, the field list, and the canonical-field contract as its producers, with the canonical field reader, the one check that a field list matches a description's field kinds in count and in kind, every description's rebuild and every encoder's check reading through it and carrying its refusal unchanged;
`encoding/abi`, authoring `IEncoderAdapter` and `IDecoderAdapter` under one versioned encoding identifier, the operation returning the encoding family's type-name identifier for a canonical field-kind identifier, the declaration carrying the encoding identifier and the total mapping from each canonical field-kind identifier to its type-name identifier, and the family's mock as its producers and implementing both, carrying its settings-document type, its guard, and its settings admission, realizing the type-name mapping as the ABI table, its known-answer vectors the root type's through its description and its identifiers' encodings authored from the ABI specification, its round trip and its rejections of truncated input, trailing bytes, non-canonical padding, and a wrong element count running over the same values, and encoding the byte-string kind that carries a group element's precompile encoding as ABI `bytes` and, for the proof family's transcript descriptions, the fixed twenty-byte and 256-bit unsigned kinds, each named as a width, the kinds being `encoding/canonical_field_reader`'s, each against its own known-answer vector with the decoder rejecting a word outside the width;
`encoding/lookup_encoding_declaration`, `encoding/admit_encoding_selection`, and `encoding/admit_encoding_composition` preceding `encoding/factory`, which constructs only admitted selections and carries the family's integration test across factory, concrete, and description;
`kdf/blake3_keyed` for every off-chain derivation over a context its caller encodes through the encoding family, authoring the derivation interface and the KDF identifier and carrying its settings admission, then the family's declaration lookup, selection admission, and composition admission, then `kdf/factory`; `hash-to-scalar/keccak256` for the identity mapping and the challenge, authoring the interface that maps domain-tagged bytes to a scalar and its identifier and carrying its settings admission, then the family's declaration lookup, selection admission, and composition admission, then `hash-to-scalar/factory`.

**Entry.** Workspace and discipline bootstrap.

**Exit.** CR-10 vectors against precompile behavior including subgroup rejection and encoding edge cases on both curves, each curve member admitted through the pairing family's admission functions and constructed through its factory, and each scalar type's draw from uniform bytes landing in its field; each pairing concrete encoding the pairing of the two generators to the identifier's definition executed in its tests, the two libraries of each curve encoding the same pairing to the same bytes and returning the same scalar field order and the same outside-the-subgroup encodings, each of which its own decoder refuses, and composition admission refusing a selection whose required target-group encoding identifier no composed declaration provides; CR-02 corruption of roots, paths, chunks, lengths, and order detected across construction, verification, and challenge, through the hashing family's admission functions and factory, which admits a member by the commitment-scheme identifier alone and whose family carries no digest operation; the ABI concrete encoding the root type through its description and the identifiers to their known-answer vectors, round-tripping them, and rejecting malformed input, and encoding a byte string and each width kind to its known-answer vector, and returning for every canonical field-kind identifier its encoding-owned type-name identifier, the fixed twenty-byte kind's identifier rendered as `bytes20` at the generated-source boundary; CR-11's derivation and hash-to-scalar produce the specification's outputs with context strings, serialization, and output lengths frozen, matching known-answer vectors from an independent implementation; the benchmark's measurements recorded over every pairing member.

**Deliverables.** The pairing, hashing, encoding, KDF, and hash-to-scalar crates, each family's identifier types, settings admission, and three family-owned functions among them; the domain's identifier modules the derivation context composes; the benchmark record.

**Owner.** Cryptography implementer.

**Unblocks.** Credential KEM, envelope, delivery proof.

### Credential KEM

**Purpose.** The construction that makes decryption capability native and distinct per interval, and the wrap that lets every live set open one ciphertext.

**Scope.** `kem/bb1_depth_one`, implementing setup, issue, rerandomize, validity, encapsulate, well-formedness, and decapsulate as one adapter, owning its parameter-set, credential, capsule, and identity-element types, and authoring `ICredentialKemAdapter` with its associated types, the identity-scope declaration carrying the identity mapping's domain tag, and the family's mock as its producers, carrying its settings-document type, its guard, and its settings admission; `kem/lookup_kem_declaration`, `kem/admit_kem_selection`, and `kem/admit_kem_composition`, the family's declaration lookup, selection admission, and composition admission comparing identity-scope, pairing, and encoding declarations before any construction, each preceding `kem/factory`, which constructs only admitted selections; `workflows/sidecar/derivation_context` and its description, the derivation context composed of the hashing family's root type as its asset component with the domain's identifier modules, which create the workflows crate; `workflows/sidecar/wrap` and `workflows/sidecar/unwrap`, the wrapping key derived through the KDF family's admitted selection and the XOR wrap and unwrap of a piece-group key, which create no crate. The KEM's asset-scope identity bytes are the root's canonical field through its description.

**Entry.** Pairing adapters and key derivation, with every pairing concrete's definition test passing, since the encapsulated value's bytes are that test's subject.

**Exit.** CR-08 algebraic properties: cross-holder agreement on the encapsulated value, cross-set agreement with two independently generated parameter sets unwrapping one piece-group key through the wrap, rerandomized-credential validity, cross-entitlement non-convertibility under the entitlement scope, holder-authored grants under the asset scope, malformed-capsule rejection, trivial identity-element refusal; CD-08's scope declaration refused by the family's selection or composition admission where it must be.

**Deliverables.** The KEM crate, its settings admission and three family-owned functions among its modules; the workflows crate with the derivation context, its description, and the sidecar wrap and unwrap.

**Owner.** Cryptography implementer.

**Unblocks.** Envelope, whose wrap and unwrap carry the KEM's `CredentialComponents`; delivery proof.

### Envelope

**Purpose.** Delivery of a credential to a recipient's registered keys and nothing else's.

**Scope.** `envelope/possession_statement`, the family-owned description of a proof of possession's statement, the key's and the commitment's precompile encodings as byte strings, implementing the encoding contract and creating the envelope crate; `envelope/pairing_elgamal`, implementing key generation under proofs of possession, wrap, and unwrap as one adapter with identity-element and shared-secret rejection, each proof's challenge the hash-to-scalar mapping of the encoded possession statement, the key pair's secrets drawn by the caller and the coins and nonces through the randomness family, wrapping and unwrapping the KEM's `CredentialComponents`, owning its key-pair, public-key, possession, and envelope types, and authoring `IKeyAgreementAdapter` with those types as associated types with their fallible constructors, a recipient's public keys admitted only with their proof of possession, the envelope-algebra declaration carrying the proofs of possession's domain tags, and the family's mock as its producers, carrying its settings-document type, its guard, and its settings admission; `envelope/lookup_envelope_declaration`, `envelope/admit_envelope_selection`, and `envelope/admit_envelope_composition`, the family's declaration lookup, selection admission, and composition admission comparing envelope-algebra declarations before any construction, each preceding `envelope/factory`, which constructs only admitted selections. The validity check of an unwrapped credential belongs to the credential engine through the KEM family's admitted selection, not to the envelope.

**Entry.** Pairing adapters and key derivation, including the encoding contract's byte-string kind; credential KEM, whose admitted family surfaces supply `CredentialComponents`.

**Exit.** CR-04: independent coins, shared or identity secrets rejected, exact round trip, swapped keys or elements yield decryption failure or an invalid credential; LC-08's registration hygiene at the algebra level.

**Deliverables.** The envelope crate, its settings admission and three family-owned functions among its modules.

**Owner.** Cryptography implementer.

**Unblocks.** Delivery proof.

### Delivery proof and on-chain verifier

**Purpose.** The proof the contract verifies before it records an interval, in Rust as the reference and in Solidity as the shipped contract, bit for bit.

**Scope.** `chain/evm_forms`, the EVM suite's identity, entitlement, interval, and chain-identifier forms over the kinds the suite's schemas fix as a private concrete of the chain family, with the identity form's address derivation over a public key's canonical bytes and its scheme identifier, the signature family's scheme-identifier type taken through that family's `provides` and no duplicate type held, the signed-intent form as EIP-712 typed data over the call's fields, chain, contract, nonce, and expiry, shared by every EVM chain concrete, constructed over a borrowed encoder from which it reads each typed-data member's encoding-owned type-name identifier, and declaring the encoding identifier it requires,
consuming the canonical-field contract and authoring, in `adapters/chain`'s factory module, the forms portion of the family's declaration, naming address derivation and the signed-intent form, a forms identifier with adapter and interface versions, the family's form interface, declaring both beside the identity, entitlement, interval, and chain-identifier forms and each form bounded by that contract, and the family's mock, covering both, and creating the `adapters/chain` crate;
`chain/evm_forms/settings`, the forms concrete's settings-document type, its guard, and the admission of its own settings;
`chain/lookup_chain_declaration`, `chain/admit_chain_selection`, and `chain/admit_chain_composition`, the chain family's initial declaration lookup, selection admission, and composition admission for the forms concrete, following `chain/evm_forms/settings` and preceding `chain/create_chain_forms`; the lookup reads the forms declaration without construction, the selection admission narrows the untrusted settings through the forms guard and invokes its settings admission, and the composition admission compares the forms and encoding requirements as typed identifiers;
`chain/create_chain_forms`, the family-owned function constructing the forms concrete the admitted selection names over the borrowed encoder it takes in its deps, the forms selection routed through the family's selection admission, the forms identifier the hash-card carries as an explicit field, supplied by the configuration in the harness and read from the deployment record in the daemon, admitted through that selection, its composition admission comparing the suite-required forms and encoding identifiers as typed values against the resolved declarations before it constructs forms or touches a side effect, and handing the admitted forms concrete to a consumer generic over the form interface, its own module because the factory holds one function and that function constructs the chain concrete;
`proof/mint_statement` and `proof/transfer_statement`, the family-owned descriptions of the mint and transfer transcripts under delivery-statement version one, the transcripts' asset field the root type taken through its description, carrying the purposes as declared data with their sixteen-bit codes and relations, generic over the chain family's identity, entitlement, interval, and chain-identifier forms with the entitlement contract in the identity form, the first creating the `adapters/proof` crate;
`proof/schnorr_fs/challenge`, the Fiat–Shamir challenge the concrete owns, the hash-to-scalar mapping of a transcript's encoding under the proof's domain tag through the encoding and hash-to-scalar concretes the hash-card's identifiers name;
`proof/schnorr_fs`, the mint and transfer provers and the verifier in both forms as one adapter, declaring both forms and authoring `IDeliveryProofAdapter`, the algebraic statement types over the pairing's elements, the declaration of supported envelope algebras, verifier forms, and delivery-statement versions and of the challenge and weight domain tags, and the family's mock as its producers;
`proof/schnorr_fs/settings`, the concrete's settings-document type, its guard, and the admission of its own settings;
`proof/lookup_proof_declaration`, `proof/admit_proof_selection`, and `proof/admit_proof_composition`, the family's declaration lookup, selection admission, and composition admission comparing the resolved envelope's algebra, the verifier form the resolved pairing declares, and the delivery-statement version the hash-card names before any construction;
`proof/factory`, calling `proof/admit_proof_composition` before it constructs anything and constructing only the member the admitted selection names;
`harness-crypto/delivery/mint`, the mint delivery from a typed mint context and an `AdmittedKemIdentity`, setting up the parameter set, deriving the identity element, issuing, generating and admitting the recipient's keys, wrapping, and proving the mint, returning the statement and proof, the parameter set, identity element, and credential, copies of the witnesses the mint prover took, the recipient as a `DeliveryHolder`, and its uniforms, owning `AdmittedKemIdentity` and `DeliveryHolder`, adding a `Clone` impl for `DeliveryContext` in the proof family's factory interface and `Clone` and `Copy` to the KEM's `SetupScope` and `KemIdentity`, and creating the `delivery` module;
`harness-crypto/delivery/transfer`, the transfer delivery from a typed transfer context, the parameter set, the identity element, and a holder, unwrapping, rerandomising, generating and admitting the buyer's keys, wrapping, and proving the transfer, returning the statement and proof, the credential, copies of the witnesses the transfer prover took, the buyer as a holder the next transfer takes, and its uniforms;
these two own every delivery the harness proves, encodes, or times, and no other ticket composes a delivery step;
the functions the Solidity generate concrete owns, each its own ticket ahead of the adapter:
`harness-crypto/generate/solidity/render`, rendering a constants library from a name and an ordered list of named entries and a vector-record library from a name and, per vector kind, a struct with its members, a decode function, and a path constant, under the license identifier and the compiler version its params carry, and owning the entry type, the record-declaration type, and the validated name types, each record member carrying its name, its canonical field-kind identifier, and its encoding-owned type-name identifier resolved by the caller, with the identifier's declared ABI spelling rendered only at the generated-file boundary, so that it stays pure and reads no adapter;
`harness-crypto/generate/solidity/constants`, each family's domain tags read from its declaration, the delivery-statement version, the purpose codes, both transcripts' field sequences as encoding-owned type-name identifiers read from the borrowed encoder, with a fixed twenty-byte kind rendered as `bytes20` only in the generated source, the signed-intent form's key-identifier domain tag, intent domain name and version, intent struct name, and member names read from the declaration of the forms it is handed, the type-name identifiers of the nonce and expiry kinds and of each canonical field-kind identifier read from the borrowed encoder, and the cell's values record it returns — the curve and precompile-encoding identifiers the pairing declares rendered by their declared wire codes at this file boundary, the scalar field's order, the generators' encodings, and whether the verifier has second-group arithmetic — owning its record type, its canonical description, and its member names;
and the vector functions, each returning typed records with their canonical descriptions and member names:
`harness-crypto/generate/solidity/group_vectors`, the group-operation and pairing-product records in the form the pairing declares, each kind drawn, and declaring `VectorCount` and `MsmTermCount` in the generate family's factory interface, the first source file requiring them;
`harness-crypto/generate/solidity/rejection_vectors`, candidate encodings the suite must refuse, each naming what it decodes into, the outside-the-subgroup candidates the pairing family's, an enumerated kind;
`harness-crypto/generate/solidity/mapping_vectors`, the hash-to-scalar records, drawn, and the identity-mapping records, one per `AdmittedKemIdentity` it is handed, admitting no identity;
`harness-crypto/generate/solidity/possession_vectors`, keys with their proofs' components, drawn;
`harness-crypto/generate/solidity/challenge_vectors`, standalone mint and transfer transcripts over the typed contexts it is handed, with their encodings and challenges, drawn, rebuilding no form and declaring `CanonicalDeliveryContext` and `DeliveryContextField` until their first consumers move them;
and `harness-crypto/generate/solidity/proof_vectors`, mint and transfer statements with their proofs' components, drawn, a mint record encoding one mint delivery and a transfer record a mint delivery followed by a transfer delivery, composing no delivery step, drawing nothing, and admitting nothing;
the contract suite identity the generate and verifier families take as a typed identifier declared in the suite's own declaration — the EVM suite's declared identity authored by its first source file `contracts/evm/ParameterSets` — with no domain enumeration of suites and no text copy held;
and `harness-crypto/generate/solidity/settings`, the admission of the concrete's own settings in the trusted form, refusing a value for a setting the declaration fixes and a settable value outside its declared bounds and taking the declared default where the document supplies none, owning the concrete's settings type, its settings-document type, and the guard narrowing to it, and authoring, as its producers, the generate family's declaration type with the member identifier, the vector-kind identifier, each kind's sizing and second-group gate, and each setting's statement, and the concrete's declaration constant;
`harness-crypto/generate/solidity`, the adapter generic over the form interface, constructed over the adapters and declarations its caller obtained through the pairing, credential KEM, key-agreement, proof, hash-to-scalar, encoding, and randomness families' admitted selections and factories over the forms that function hands it, calling each function it owns with each drawn kind's configured count, resolving every record member's encoding-owned type-name identifier from its borrowed encoder before calling `render`, which renders the identifier's declared ABI spelling only at the generated-file boundary, and declaring the encoding identifier it requires, encoding each record through the encoder into one line, writing one vector file per kind its declaration names in the cell directory, and the constants library and the vector-record library in the generated directory outside every cell directory, the vector-record library's path constant holding each kind's file name, every file and library name its per-kind settings and params carry, with the typed mint and transfer contexts, the mapping identities, and the proof identity as `AdmittedKemIdentity` values arriving as payload, admitting no identity and rebuilding no form,
and authoring the generate family's interface with its associated `Forms` type, the per-kind settings type, and the interface naming `VectorCount` and `MsmTermCount` as `group_vectors` declares them, over the declaration `harness-crypto/generate/solidity/settings` authors, which names the kinds it emits with each kind's sizing, the enumerated kind holding the cell's values record among them,
`harness-crypto/generate/admit_generate_selection`, the generate family's selection admission calling the selected member's settings admission over that member's own table;
`harness-crypto/generate/lookup_generate_declaration`, the family's declaration lookup;
and `harness-crypto/generate/admit_generate_composition`, admitting the chain forms identifier and the encoding identifier the member requires against the resolved forms' and the borrowed encoder's declarations before construction,
then `harness-crypto/generate/factory`, admitting a configuration only when its per-kind settings name exactly the declared kinds and calling `harness-crypto/generate/admit_generate_composition` before constructing anything, its consumer trait generic over the forms, and taking no scope;
`contracts/evm/ParameterSets`, the records the registry consumes, the suite's first source file in its declared source language, authoring the suite's declared identity the deployment records key on,
creating `contracts/evm/foundry.toml`, granting read access to the generated directory and the ABI description directory and write access to the deployments directory alone with `ffi` disabled and compiling through the IR pipeline with `via_ir = true`, and the Solidity continuous-integration definition with `forge build`, `forge fmt --check`, and `forge test`, each authored once and complete;
`domain/handshake_key_commitment`, the commitment to an identity's bound handshake key under the signature family's scheme identifier taken through that family's `provides`, following `signature/factory`;
`chain/base`, the Base concrete through `alloy`, authoring, as its producers, the registry's and the entitlement interface's language-neutral ABI descriptions under `contracts/evm/abi/` and binding them through `sol!`, with Base's tier mapping, the entitlement-state views, and the event and calldata reader, binding the forms `create_chain_forms` resolves and authoring none,
authoring the chain interface, `ISettlementAdapter`, `IEntitlementStateAdapter`, the attempt context, the family's member identifier, selection, declaration remainder, declared set, consumer surface, and mock, and the estimated fee of a transaction its caller hands it, never settled, as the family's description,
`chain/base/settings` owning its settings, its settings-document type and guard, and the admission of its own settings, its declaration stating the endpoint, the gas price oracle, the chain identifier, the entitlement contract, and the precompile encodings, each fixed or settable within declared bounds with a default, the gas price oracle reaching no family surface;
`chain/lookup_chain_declaration`, `chain/admit_chain_selection`, and `chain/admit_chain_composition` taking their Base increments after `chain/base/settings`: the lookup adding the `base` member's declaration branch, the selection admission narrowing Base's untrusted settings through its guard and invoking its settings admission, and the composition admission comparing the Base member's forms and precompile identifiers with the forms' declarations, each preceding `chain/factory`;
then `chain/factory`, calling `chain/admit_chain_composition` before it constructs anything and constructing the chain concrete the admitted selection names, with its `base` branch, and handing it to the family's consumer;
`harness-crypto/verifier/evm`, authoring, as its producers, the delivery verifier's language-neutral ABI description `contracts/evm/abi/IDeliveryVerifier.json`, binding it through `alloy`'s `sol!`, and authoring no contract source and no toolchain file, consuming the chain family through `chain/factory`, the EVM concrete, named for the ABI it speaks, over the deployed verifier through `alloy`, generic over the form interface, constructed over the borrowed adapters and declarations its caller hands it and calling no factory, its statements' context values the canonical field values its params carry, reaching the deployed verifier at the endpoint of the chain adapter its factory hands it, the deployed location carried in the factory's params beside the selection, reading acceptance and cost, the cost carried as the family's cost description, which carries the chain family's estimated-fee description, and authoring the verifier family's interface, declaration, and mock, its settings-document type, its guard, and its settings admission at `harness-crypto/verifier/evm/settings`;
`harness-crypto/verifier/admit_verifier_selection`, the verifier family's selection admission calling the selected member's settings admission;
`harness-crypto/verifier/lookup_verifier_declaration`, the family's declaration lookup;
and `harness-crypto/verifier/admit_verifier_composition`, admitting the chain, forms, encoding, curve, verifier-form, and suite identifiers the member requires against the resolved chain, forms, encoder, pairing, and generate declarations, each requirement stated in its providing family's value identifier and never naming another family's member, before construction,
then `harness-crypto/verifier/factory`, calling `harness-crypto/verifier/admit_verifier_composition` before constructing anything and constructing the concrete over the chain adapter it is handed;
`harness-crypto/measure`, size, timing, and gas capture per cell, an adapter over the operating system's monotonic clock as `harness-crypto/benchmark` is, each iteration taking fresh inputs from the delivery functions untimed and timing only `prove_mint`, `prove_transfer`, `verify` for each relation, or `decapsulate` over a capsule it encapsulates, each the mean over the configured iteration count, the capsule, envelope, and each relation's proof bytes from the pairing's encoders over the objects it produced, and its own mint and transfer submitted through the admitted verifier member, an accepted delivery's cost carried as the verifier family's cost description;
`harness-crypto/report`, the release-evidence report written under the destinations its params carry;
`harness-crypto/config`, the typed configuration the harness owns with a shipped default for every value it passes as params or payload — the matrix's lists excepted — the member and value identifier per family, the identity scope, the statements' context values, the identities admitted under that scope, an `AssetIdentityHash` under the asset scope and canonical bytes under the entitlement scope, the per-kind vector settings, the generated libraries' names, the Foundry project root and the generated directory relative to it, the directory `contracts/evm/foundry.toml` grants, the Solidity compiler version and license identifier the generated libraries open with, the matrix's lists of pairing members, target-group encoding identifiers, and encodings, each the family's real declared set where the configuration carries none, one verifier selection, the deployment records, of which the shipped defaults hold none, the measurement's settings, the report's destinations, and the output destinations among them, overridable from a configuration file and from command-line flags treated as untrusted documents each identifier's owner admits, following every ticket whose values it carries,
composed over `harness-crypto/config/layers`, `harness-crypto/config/admit_selections`, `harness-crypto/config/admit_statements`, which admits every identity once under the scope `admit_selections` admitted, admits the deployment records, a map keyed by each contract suite's declared identity and the pairing-owned curve identifier, value by value, a malformed address refused with its key, admitting a declared curve identifier without a record for the initial generation run and refusing a key naming no declared curve identifier through the pairing owner's typed error carrying that text, and moves `CanonicalDeliveryContext` out of `harness-crypto/generate/solidity/challenge_vectors`, `harness-crypto/config/admit_generate`, and `harness-crypto/config/admit_output`;
`harness-crypto/run/admit_contexts`, admitting a canonical context, the chain identifier and the entitlement contract `harness-crypto/run/chain_consumer` hands it, and its purpose into a typed `DeliveryContext` once, where the forms are resolved, a form's refusal returned with its field, and moving `DeliveryContextField` out of `harness-crypto/generate/solidity/challenge_vectors`;
the run's consumers, one per family surface it uses, each its own ticket innermost first:
`harness-crypto/run/generate_consumer`, calling `emit` with the typed contexts and the admitted identities;
`harness-crypto/run/verifier_consumer`, calling `harness-crypto/measure` over the verifier concrete and the adapters and declarations it carries, with the typed contexts, the proof identity, and the configured iteration count;
`harness-crypto/run/delivery_proof_consumer`, calling the generate factory with `harness-crypto/run/generate_consumer`, then calling the verifier factory with the configured verifier selection over the borrowed chain it carries and `harness-crypto/run/verifier_consumer`, the deployment record it was handed carried beside the selection in the factory's params as the deployed location, where it was handed none returning the refusal naming the curve identifier after the generate factory's call has written the files;
`harness-crypto/run/key_agreement_consumer`, carrying the typed contexts and admitted identities;
`harness-crypto/run/kem_consumer`, pairing each admitted identity with its setup scope as an `AdmittedKemIdentity` and carrying them with the typed contexts;
`harness-crypto/run/chain_consumer`, receiving the chain, reading the chain identifier and the entitlement contract from it, admitting the mint and transfer contexts through `harness-crypto/run/admit_contexts`, and calling the KEM family's admitted selection and factory, carrying the borrowed chain inward;
`harness-crypto/run/chain_forms_consumer`, calling the chain factory with the configured chain selection over the forms it received, its consumer `harness-crypto/run/chain_consumer`;
and `harness-crypto/run/encoding_consumer`; each carrying the configured verifier selection, the deployment-record lookup's result, and the measurement settings to the consumer it constructs; and `harness-crypto/run/pairing_consumer`, handed a cell's concrete, reading the curve identifier from the received pairing's declaration and the contract suite identity the configured verifier member requires through `harness-crypto/verifier/lookup_verifier_declaration`, looking up the deployment record that identity and the curve identifier key, and carrying the result inward with the verifier selection and the cell's encoding selection, which it hands `create_encoding`;
`harness-crypto/run`, the harness's composition root and with its consumers its only caller of a family's factory, obtaining the randomness source through the randomness family's admitted selection and factory, iterating the selected pairing members in list order and, within each, the selected target-group encoding identifiers in list order, calling `create_pairing` for each such pair with the benchmark as its consumer and, for each selected encoding in list order, with the pairing consumer, the cell's encoding carried to it as its encoding selection, completing every family's composition admission before any file, deployment call, measurement, or output, no refusal ending the run, and returning the run's outcome — the benchmark's measurements per pair and, per cell, its outcome, the files written, the measurements, and the report `harness-crypto/report` records over them where the cell ran to completion, its refusal where it refused, every pair's and every cell's outcome carried once in iteration order, each naming its member identifiers, in the error arm when any is refused — its public integration test running the whole chain;
`harness-crypto/output`, writing the outcome, success or refusal rendered to canonical spellings or declared wire codes only at that boundary, to each configured destination, standard output, a file, or both;
`harness-crypto/main`, the binary entry, calling the configuration, the run, and the output in turn and setting the exit status;
`contracts/evm/PairingLib` over EIP-196, EIP-197, and EIP-2537 with contract-side subgroup checks, its constants read from the committed constants library covering the values that are the same for every cell, and its tests run once per cell directory they enumerate under the generated directory, reading that directory's values and vector files through the generated vector-record library;
`contracts/evm/DeliveryVerifier`, authoring, as its producers, the delivery verifier's Solidity interface `contracts/evm/src/IDeliveryVerifier.sol`, realized against the ABI description `harness-crypto/verifier/evm` authors, with its interface test, implementing it, and declaring the delivery-statement versions and verifier form under which the adapter registry binds it, receiving the scalar field's order and the generators' encodings as constructor arguments, and its tests run once per cell directory they enumerate under the generated directory;
the contract tickets that verify or assemble signed intents, each building under that Foundry configuration, authoring its ABI description, and reading the key-identifier tag, the intent form's domain, struct, and member names, and the type names from the committed constants library: `contracts/evm/IdentityAccount`, the ERC-1271 identity account with its device registry, signer admission, re-scoping, and revocation, the `isDeviceAdmitted` view, and signed-intent verification, authoring, as its producers, the signature verifier interface every on-chain signature check passes through per declared signature-scheme identifier;
`contracts/evm/Binding`, binding the handshake key's scheme identifier and commitment and creating the account;
and `contracts/evm/Registry`, with asset and alias records, deployments, sidecars, parameter sets, source keys, grant requests, and `evaluateAuthorization` with its batch, realizing its interface against the ABI description `chain/base` authors, and authoring, as its producers, the entitlement token form's interface with its views and settlement events, realized against the ABI description `chain/base` authors, and the identity adapter and attestation verifier interfaces;
`contracts/evm/deploy` to Anvil and Base Sepolia once per distinct curve identifier in the verifier form its values state, emitting each deployed address as a deployment record keyed by the suite's declared identity and that pairing-owned curve identifier, a runtime output the harness's later runs read as a configuration layer over the shipped defaults, the layer's table holding one key per distinct curve identifier rendered by its declared wire code beneath the suite's declared identity, and carrying the milestone's commit with the generated directory and the deployment record.
The delivery functions follow `proof/factory` and precede `harness-crypto/generate/solidity/render`, every function the generate concrete owns following `harness-crypto/delivery/mint`;
`contracts/evm/ParameterSets` follows `workspace/ci`, `chain/evm_forms` follows `encoding/factory` and `signature/factory`, the chain family's initial declaration lookup, selection admission, and composition admission follow `chain/evm_forms/settings`, `chain/create_chain_forms` follows them, `domain/handshake_key_commitment` follows `signature/factory`, `chain/base` follows `domain/handshake_key_commitment` and `chain/create_chain_forms`, the chain family's functions take their Base increments after `chain/base/settings`, `chain/factory` follows those increments, and `harness-crypto/verifier/evm` follows `chain/factory`, so nothing on the path to `chain/factory` needs a harness ticket at or after the verifier client;
the configuration's admission functions follow the tickets whose types they admit, each family's selection admission, declaration lookup, and composition admission following its concrete and preceding its factory, `harness-crypto/config/admit_selections` and `harness-crypto/config/admit_generate` following the selection admissions they call, and `harness-crypto/config/admit_output` following `harness-crypto/report`; `harness-crypto/run/admit_contexts` follows `harness-crypto/config`, and the run's consumers follow it;
`contracts/evm/PairingLib` and `contracts/evm/DeliveryVerifier` follow `harness-crypto/main`, `contracts/evm/IdentityAccount` follows `contracts/evm/DeliveryVerifier`, `contracts/evm/Binding` follows `contracts/evm/IdentityAccount`, `contracts/evm/Registry` follows `contracts/evm/Binding` and `contracts/evm/ParameterSets`, and `contracts/evm/deploy` follows them; the generated directory, the deployed verifier, and the measured defaults are runtime inputs, held in the dependency map's Runtime sequence.

**Entry.** Credential KEM and envelope; signature services, whose `signature/factory` exports the scheme identifier the chain family's identity form and the handshake key commitment take through the family's `provides`, running beside this grouping ahead of `chain/evm_forms`.

**Exit.** CR-09 in both verifier forms with every statement field and response mutated and replay across settlements rejected;
CD-03 on each curve form with record-contradicting statements rejected;
the Rust verifier and the deployed contract agree on every vector;
the hash-to-scalar identity-mapping and challenge vectors mirrored to the suite's language by the generator and reproduced by the deployed verifier;
the proof family's composition admission refuses each curve and verifier-form combination it must and a hash-card naming a delivery-statement version no description supports;
the chain family's composition admission refuses a forms selection whose identifier the suite does not require and hands the admitted one to a consumer that never names it;
every mint and transfer delivery's statement and proof accepted by the reference verifier, its returned witnesses proving that statement again to a proof the reference verifier accepts, and a transfer from a transfer's returned holder accepted likewise;
each configured context admitted once into its forms and each identity once under the admitted scope, a refused form reported with its field and a refused identity with its document path;
the harness run composes every family through its admission surfaces and factory from the configuration's values, with a file and a flag override each shown to replace a default;
the run's outcome, success or refusal, written to standard output, to a configured file, and to both, as the configuration names, with the benchmark's measurements for every selected member in a success;
the generated constants rendering every identity field of a transcript as `bytes20`, which the deployed verifier encodes it as;
the generate family's selection admission refusing per-kind settings that omit a declared kind, name an undeclared one, or give a count to a kind that is not drawn;
the identity account refusing foreign, expired, replayed, revoked-signer, and out-of-permission intents, and the registry's `evaluateAuthorization` returning each of its three states with its batch agreeing with individual evaluation, through Foundry tests;
the chain family's composition admission refusing a forms selection whose identifier the chain member's declaration does not list before constructing anything, the chain concrete's settings admission refusing a setting outside its declared bounds and a value for a setting the chain fixes, and the Base concrete's tier mapping and envelope recovery from events and calldata proven against the bound ABI descriptions;
each declared kind's vector file in each cell directory holding its configured count, its enumerated set, or one record per configured input, every line decoding through the vector-record library's function for that kind; the generated directory written by that run before the pairing library's tests read it and carried by the milestone's commit; a cell whose identifiers its pairing member does not declare refused by the pairing family's composition admission and reported as incompatible, every other cell still run, and a run whose cells are all completed or incompatible succeeding; the verifier deployed on Base Sepolia, one address per distinct curve identifier emitted as the deployment record.

**Deliverables.** The proof crate with its transcript descriptions, the chain crate with its form interface, its EVM forms concrete and its settings, its family-owned forms function, its Base concrete, its selection admission, declaration lookup, and composition admission, and its factory, the domain's handshake key commitment module taking the signature family's scheme identifier through `provides`, the suite's declared identity authored by its first source file, the delivery module with its mint and transfer functions, the generate and verifier families with their Solidity and EVM concretes and their settings admission, declaration lookup, selection admission, and composition admission, the measurement and report modules, the harness configuration with its identity admission, the run with its context admission, the output, and the binary,
the committed generated directory, the delivery verifier's ABI description, authored by the verifier client, and its Solidity interface, authored by the on-chain verifier, the Foundry configuration and the Solidity continuous-integration definition, created by the parameter-set contract, the registry's and the entitlement interface's ABI descriptions, authored by the Base concrete, the signature verifier, entitlement, identity adapter, and attestation verifier interfaces, the identity account, binding, parameter-set, and registry contracts with their ABI descriptions, the Solidity pairing library and the on-chain verifier, the Base Sepolia deployment record.

**Owner.** Cryptography implementer with the contract implementer.

**Unblocks.** Harness measurement; registry and entitlement contracts; the chain and submission families.

### Harness measurement and report

**Purpose.** Measure what nothing has measured, fix the parameters that gate every node that encrypts, and calibrate throughput.

**Scope.** No new ticket. The harness binary's next run after `contracts/evm/deploy`, with the deployment-record layer — each record keyed by its typed suite identity and pairing-owned curve identifier, rendered to declared wire codes at the layer file's boundary and admitted through the owning configuration boundary — layered over its shipped defaults, verifies against the deployed verifiers, measures, and writes the report's typed measurement and outcome values to its configured destinations, and the end-to-end tests that read the deployment records run against them here, outside the ordinary `cargo test` cycle; the release evidence, the throughput record, and the re-map of the remaining protocol core milestones follow from what it writes.

**Entry.** Delivery proof and on-chain verifier.

**Exit.** CD-07 and AS-21: capsule, envelope, and proof sizes; decapsulation time per piece group; proof generation and verification time; delivery cost for a mint and a transfer through the verifier family's cost description, which carries the chain family's estimated-fee description, on both curve members, the L1 fee noted as Sepolia's; the measurements, the piece-group size and curve-identifier defaults in force, and any adjustment the data supports recorded as typed values in release evidence; tickets closed per unit of time recorded across the grouping.

**Deliverables.** The release-evidence document; the throughput record; the first evidence-based estimate of the remaining work, produced by re-mapping the remaining protocol core milestones to tickets.

**Owner.** Cryptography implementer; project lead for the default adjustment, the re-map, and the funding decision.

**Unblocks.** The re-map and the funding stop criterion.

## Protocol core and contract suite grouping

The signature and swarm milestones are at ticket resolution and run beside the harness from the encoding and hashing families' closure onward, since their first tickets encode and hash and they depend on nothing the harness measures; the hashing family is built at the head of the harness grouping, in pairing adapters and key derivation, because its root type is the asset's identity and the first type the encoding family describes; the tickets on the path to `chain/factory`, `domain/handshake_key_commitment`, `chain/base`, the chain family's selection admission, declaration lookup, and composition admission, and `chain/factory`, are at ticket resolution in the delivery proof and on-chain verifier milestone, ahead of `harness-crypto/verifier/evm`, because the harness's verifier client, configuration, and run consume the chain family through its factory; the remaining milestones are re-mapped to tickets when `contracts/evm/deploy` commits.

### Domain model

**Purpose.** The protocol's remaining types, each fixed where it is first consumed, with the canonical description every encoded one needs.

**Scope.** The protocol's remaining types, each authored in the node of the file that first consumes it and housed in the module that implements it, the identifier modules being harness tickets and the derivation context a sidecar-layer ticket; the `domain` crate holds only the types its own modules implement and no asset identity, no derivation context, and no family's closed-set identifier; for each encoded type housed in a crate the encoding family depends on, its canonical description as the encoding family's module `encoding/<type>`, implementing the family's canonical-field contract for that type, one module per type, authored after the type and before the first ticket that encodes it; a type housed in a crate that depends on the encoding family carries its canonical description as a module of that crate, as `envelope/possession_statement`, `proof/mint_statement`, and `workflows/sidecar/derivation_context` do; `domain/handshake_key_commitment`, which the attempt context `chain/base` authors carries, is mapped in the delivery proof and on-chain verifier milestone.

**Entry.** Pairing adapters and key derivation, credential KEM, envelope, and delivery proof and on-chain verifier, whose families own the types the remaining types carry.

**Exit.** Every remaining type constructed only through its fallible constructor; every encoded type's canonical description encoding and decoding it through the encoding family's admitted member.

**Deliverables.** The remaining domain types and their canonical descriptions.

**Owner.** The owner of each type's first consumer.

**Unblocks.** Payload cipher and sidecar layer; registry and entitlement contracts; durable jobs and configuration.

### Payload cipher and sidecar layer

**Purpose.** The symmetric layer, and the sidecar that lets every live parameter set open one ciphertext.

**Scope.** `cipher/aes_ctr` with the 64/64 counter layout, continuous-stream addressing, extent and index bounds, authoring `IPayloadCipherAdapter`, the counter-layout and addressable-extent value identifiers and declarations, and the family's mock as its producers, carrying its settings-document type, its guard, and its settings admission; the family's declaration lookup, selection admission, and composition admission, each preceding `cipher/factory`, which constructs only admitted selections; `workflows/sidecar/build`, per live parameter set a capsule and wrapped piece-group key per group committed by that set's Bao root, each sidecar its own object; `workflows/sidecar/validate`, manifest, hash-card, and sidecar validation in the ordering the specification requires; the decapsulation-to-cipher seam; the `sample_deployment` generator for the site demonstration, which needs the cipher and so lives here rather than in the harness.

**Entry.** Domain model; pairing adapters and key derivation, for the hashing family; the credential KEM milestone, for the derivation context and the wrap. The `sample_deployment` generator carries the configured piece-group size default.

**Exit.** CR-01 vectors including maximum-valid counters and overflow rejection; CR-06 fuzzing with chain and custody spies proving validation precedes any credential exercise; a two-set sidecar opening one ciphertext under a credential of either set; the sample deployment bundled for the WebAssembly build.

**Deliverables.** The cipher crate, its settings admission and three family-owned functions among its modules; the validation workflow; the sample deployment.

**Owner.** Cryptography implementer.

**Unblocks.** Decryption pipeline, First Finder engine, the site demonstration.

### Signature services

**Purpose.** Per-layer signatures behind one family that resolves a scheme identifier per layer.

**Scope.** `signature/ed25519` with domain separation and complete-message binding, authoring `ISignatureAdapter`, signing and verification over the public-key and signature types the concrete supplies as associated types, no key recovery and no address derivation in the trait, the per-layer declaration of scheme identifier, the family's member identifier, the family's declaration type, and the family's scheme-identifier type with its `uint8` wire mapping authored in the family's factory module and exported through `provides`, key and signature lengths, recovery support, and layers served, and the family's mock as its producers, carrying its settings-document type, its guard, and its settings admission; `signature/secp256k1` through `alloy`, declaring recovery support and signing the bytes the chain family's signed-intent form hands it, owning no message format and no address derivation, and carrying its own settings admission; `signature/lookup_signature_declaration`, `signature/admit_signature_selection`, and `signature/admit_signature_composition`, the family's declaration lookup, selection admission, and composition admission comparing the selected member's declared scheme identifier, layers served, key and signature lengths, and recovery support against the requirements the composed declarations state, each preceding `signature/factory`, which calls composition admission before it constructs anything, resolves a scheme identifier per layer with a branch per concrete the family holds at that ticket, and carries the milestone's integration test and commit; the family's lattice concrete, `signature/ml_dsa_65`, and the factory's branch for it are the post-quantum signature concrete milestone's. The binding schema with the DID Document types belongs to the identity family and is authored in the identity and custody milestone.

**Entry.** The encoding family's closure at `encoding/factory`, inside pairing adapters and key derivation. Independent of the rest of the harness and of the cipher milestone.

**Exit.** CR-03: valid signatures verified; cross-domain, cross-layer, truncated-context, wrong-scheme, and replay substitutions rejected, each scheme identifier resolved through the family's admission functions and factory for its layer.

**Deliverables.** The signature crate, its member and scheme identifiers, its concretes' settings admission, and its three family-owned functions among its modules.

**Owner.** Cryptography implementer.

**Unblocks.** Identity and custody; contract binding surface.

### Swarm transport and seed host

**Purpose.** Bytes moving between machines with nothing readable in transit, provable with no chain and no KEM.

**Scope.** `transport/rqbit`, over the overlay branch the workspace manifest names, with the concrete's hooks and the upstream issues opened, root-to-infohash translation, torrent creation per object with the ciphertext and each sidecar as its own object and locator, Bao verification of every completed piece after the library's SHA-1 check through the hashing family's admitted selection and factory, seeder-map peer injection, and the engine capability, authoring `ISwarmTransportAdapter`, the locator types, the family's member identifier and its locator-form and engine-capability identifier types, the declaration, and the family's mock as its producers, carrying its settings-document type, its guard, and its settings admission; the family's declaration lookup, selection admission, and composition admission, each preceding `transport/factory`; `discovery/admit_peers`, the family-owned function admitting the addresses a source yields into discovered peers, authoring `IPeerDiscoveryAdapter`, the key and peer types, the family's member identifier and its discovery-source identifier type, the declaration, and the family's mock as its producers and creating the crate; `discovery/local`; `discovery/locator_of`, the family-owned function converting a key to the transport's locator; `discovery/dht`, `discovery/pex`, and `discovery/tracker` through the transport's engine capability, each concrete carrying its own settings admission; the family-owned `discovery/aggregate`; the family's declaration lookup, selection admission, and composition admission, each preceding `discovery/factory`, the discovery family's factory function with a branch per concrete the family holds at that ticket, carrying the family's integration test across factory, concretes, and aggregation; `storage/redb`, at its first consumer, authoring the store interface and carrying its own settings admission; the family's declaration lookup, selection admission, and composition admission, each preceding `storage/factory`; `seed-host/owned` with the root-keyed ciphertext store, holding-kind identifiers, quota, eviction, and its index through the storage family's admitted selection and factory, settings passthrough for the controls the library has and enforcement of the ones it does not, authoring `ISeedHostAdapter`, the capacity and holding-kind identifier declaration, and the family's mock, carrying its own settings admission; `seed-host/rqbit`, driving the rqbit application through its HTTP API with root-to-identifier translation and Bao custody challenges through the hashing family's admitted selection and factory, and carrying its own settings admission; the family's declaration lookup, selection admission, and composition admission, each preceding `seed-host/factory`, which carries the milestone's integration test and commit.

**Entry.** Pairing adapters and key derivation, for the hashing family and the encoding family's closure. Independent of the rest of the harness and of every contract milestone; `discovery/seeder_map`, in the chain and submission families milestone, is a further member beneath the discovery factory this milestone authors, its settings admission and the family functions' increment for it following it.

**Exit.** SW-01 through SW-07 and EC-02 as amended: retrieval through the owned host and a delegated external client with identical roots; a corrupted piece passing SHA-1 rejected by Bao before storage; discovery aggregation deterministic under conflicting and malicious results; seeding continuity across host restart while another participant retrieves; delegated custody challenges detecting false reports; resumable failure handling; the archive visible.

**Deliverables.** The transport, storage, and seed-host crates, each family's member identifier, its concretes' settings admission, and its three family-owned functions among them; the discovery crate with its peer admission, locator, local, library-backed, and aggregation modules, its member and source identifiers and admission functions, and its factory; the overlay branch; the upstream issues.

**Owner.** Daemon implementer.

**Unblocks.** Chain and submission families, for the discovery factory its seeder-map member is added to; First Finder engine; the resolution orchestrator; the project seed host.

### Post-quantum signature concrete

**Purpose.** A lattice signature scheme at the handshake and content-signing layers, so an identity can bind a key under a scheme NIST's post-quantum transition retains.

**Scope.** `signature/ml_dsa_65`, the FIPS 204 ML-DSA-65 concrete as one adapter implementing `ISignatureAdapter` through `fips204`, with key generation from a seed, signing with the randomness it draws through the randomness family, verification, domain separation carried in the scheme's context string, and complete-message binding, owning its public-key and signature types with their fallible constructors and declaring its member identifier in the family's declared real set and its scheme identifier in the family's scheme-identifier type, its key and signature lengths, no recovery support, and the layers it serves; its settings admission's further node, the further nodes of the family's declaration lookup, selection admission, and composition admission — each extended over the `ml_dsa_65` member and its declaration — and the signature factory's further node, adding the branch that constructs the concrete when the admitted selection names it, the composition admission already passed, and extending the family's integration test across it, which carries the milestone's commit. The concrete covers signatures alone, the chain layer signs with secp256k1, and no contract verifies the scheme.

**Entry.** Signature services. Independent of swarm transport and seed host, whose close at `seed-host/factory` its tickets follow, and of every contract milestone.

**Exit.** CR-03 for a scheme without recovery: valid signatures verified against the FIPS 204 known-answer vectors; cross-domain, cross-layer, truncated-context, wrong-scheme, and replay substitutions rejected; the scheme resolved through the family's admission functions and factory for its layer beside the family's other members.

**Deliverables.** The signature crate's lattice concrete, its settings admission, the family functions' increments for it, and the factory's branch.

**Owner.** Cryptography implementer.

**Unblocks.** Identity and custody, for a handshake key bound under the lattice scheme.

### Registry and entitlement contracts

**Purpose.** The ledger the protocol's decentralized properties rest on.

**Scope.** The EVM suite under `contracts/evm`, the on-chain concrete of the chain family's Base concrete, from the technical requirements' API surface, `contracts/evm/IdentityAccount`, `contracts/evm/Binding`, `contracts/evm/ParameterSets`, and `contracts/evm/Registry`, with the signature verifier, entitlement, identity adapter, and attestation verifier interfaces they author, being mapped in the delivery proof and on-chain verifier milestone and the rest following here:
registry with asset records keyed by the asset's root and append-only alias records under a declared namespace and an opaque key that no Registry field interprets, each entry binding the key to one root with the source's signed statement or its recorded absence and never rebound or removed, deployments carrying every hash-card field, the commitment-scheme identifier among them, and width-bearing fields length-carrying or committed, parameter sets, envelope keys, identity elements, and envelopes held as suite-tagged bytes with a digest and interpreted only by the delivery verifier the adapter registry binds, the sidecar-per-live-set rule, sidecar addition for later sets, attestation verification through the `IAttestationVerifier` adapter the adapter registry binds per ingest-source identifier, with the npm P-256 verifier over the source-key table as the first member and a recorded absence admitted, and later escrow deployments of an existing asset;
parameter-set liveness and retirement;
envelope-key registry with proofs of possession;
`IEntitlement` with its ERC-721 concrete carrying interval state and envelope digests and the ownership override that disables the standard transfer and approval entry points;
batched grant requests readable by holders and closed by grant or withdrawal;
locks naming their payment-asset identifier beside the price, with expiry and refund, moving payment only through `IPaymentAsset` as the adapter registry binds it per payment-asset identifier, with the native-asset member the family's real declared set holds in the MVP;
mint, deliver, grant calling the delivery verifier through its adapter registry binding;
the identity account, a contract account under ERC-1271 on the EVM suite, with its device registry keyed by scheme-tagged key identifiers verified through the signature adapter bound per signature-scheme identifier, admission, re-scoping, revocation, and the `isDeviceAdmitted` view; identity binding storing the scheme identifier and a commitment to the handshake key; every identity-bound mutation taking the acting identity and a signed intent verified through the account against a signer whose permissions cover it, with lock expiry enforced and no volume limit; `evaluateAuthorization` and its paginated batch; escrow records per deployment with provenance, attestation or its absence, and no maintainer commitment; identity binding; claim settlement state layout and verifier-key registry, each key a scheme identifier and a commitment to the key with the key in calldata and vouchers verified through the signature adapter the registry binds for the scheme identifier; `IIdentityAdapter` with the publisher-authority and escrow-identity adapters; deployment scripts to Anvil and Base Sepolia, emitting each deployed address as a typed deployment record keyed by the suite's declared identity, a runtime output read as a configuration layer.

**Entry.** Delivery proof and on-chain verifier; domain model.

**Exit.** Every transition of LC-01 exercised through Foundry tests with asserted events and views, the Rust form of that proof belonging to the chain and submission families milestone; LC-02, LC-05 including a lock naming an unbound payment-asset identifier refused and a second bound payment-asset member admitted through its family settling a lock with no change to the lock or settlement contracts, LC-07, LC-08, LC-09 including sidecar addition, LC-10, LC-11 with owner, approved address, and operator each refused an ordinary transfer, LC-12 with batched requests opened, fulfilled, withdrawn, and shown to authorize nothing, LC-13 through the identity contract account as paymaster-sponsored, relayer-submitted, and self-funded calls, with foreign, expired, replayed, revoked-signer, and out-of-permission intents refused; AS-13's proof-set acceptance and rejection on Base Sepolia.

**Deliverables.** The contract suite deployed on Base Sepolia; its deployment record; Foundry fuzz and invariant tests; static analysis in CI.

**Owner.** Contract implementer.

**Unblocks.** The adapter registry and factory; chain adapters; First Finder engine; credential delivery; claim.

### Adapter registry and factory, on chain

**Purpose.** The on-chain binding of the suite's adapters, held apart from the registry so its single project-held key can be replaced without touching entitlements.

**Scope.** The `AdapterRegistry` governed by a single project-held key and the factory that constructor-injects the identity adapters, the signature adapter per signature-scheme identifier, the attestation verifier per attestation-source identifier, the entitlement token form, the delivery verifier per delivery-statement-version and verifier-form identifiers, and the payment-asset adapter per payment-asset identifier, each binding resolved through declaration lookup and declaration-based composition admission before constructor injection and immutable once bound, the registry contracts reaching the delivery verifier and the payment-asset adapter only through those bindings; deployment scripts to Anvil and Base Sepolia, emitting each deployed address as a typed deployment record keyed by the suite's declared identity, a runtime output read as a configuration layer.

**Entry.** Registry and entitlement contracts.

**Exit.** Every binding immutable once bound and a rebinding refused; the governance key replaced with every entitlement unchanged; each bound adapter reached through the registry the factory constructs, through Foundry tests.

**Deliverables.** The adapter registry and factory deployed on Base Sepolia.

**Owner.** Contract implementer.

**Unblocks.** Acceptance.

### Chain and submission families

**Purpose.** The daemon's only view of consensus, from a quorum of configured nodes, and the one path a signed intent takes to the contracts.

**Scope.** The chain family's remaining tickets, `chain/base` and `chain/factory` with the family's selection admission, declaration lookup, and composition admission being in the delivery proof and on-chain verifier milestone: `chain/quorum_view`, the family-owned aggregation of single and paginated views as a quorum, two of three configured nodes agreeing at a common reference within `τ_soft`, stale nodes ignored, divergence and over-age views failing closed, shared by every chain member; `submission/self_funded`, the self-funded call path, authoring the interface for submitting a signed intent and its declaration and creating the `adapters/submission` crate, with its own settings admission and the family's declaration lookup, selection admission, and composition admission authored ahead of `submission/factory`, the family's factory following the sponsored members in the relayer or paymaster milestone and each later member updating every family location — its settings admission, the declaration lookup, the selection admission, the composition admission, and the factory's branch; `discovery/seeder_map`, the on-chain seeder map as a discovery-source member through the chain family's admitted selection and factory, a further member beneath the discovery factory, its node adding its settings admission, the family functions' increment for it, and its branch to that factory; intent signing through the signature family's admitted selection and factory.

**Entry.** Delivery proof and on-chain verifier, for `chain/base` and `chain/factory`; registry and entitlement contracts; signature services; swarm transport and seed host, for the discovery factory and the members and aggregation beneath it.

**Exit.** LC-01 every transition exercised through the chain family's admitted Base member with asserted events and views; LC-04 included, pending, reorganized, stale, and insufficient-tier references driving the attempt rule for a zero-price and a priced deployment; LC-06 batch parity with individual evaluation; XA-06's decision table, one timeout, one stale provider, honest heads at different heights, divergent state at one reference, a reorganization, and a prolonged stall, each with its single defined result; CD-04 recovery from chain history; SW-02 every discovery-source member, the seeder map among them, resolved through the discovery family's admission functions and factory behind the aggregation.

**Deliverables.** The chain crate completed with its quorum view; the submission crate with its self-funded member, its settings admission, and its family-owned functions; the discovery family's seeder-map member and the family functions' increment for it.

**Owner.** Chain adapter implementer.

**Unblocks.** The relayer or paymaster milestone; identity; credential delivery; every state read.

## Local daemon and package serving grouping

Re-mapped to tickets when the protocol core grouping closes. The identity milestone runs early because the installation coordinator needs it, and follows the relayer or paymaster milestone, whose submission family's admitted selection and factory it consumes. The grouping closes at the daemon composition, which follows every module it composes, the installation coordinator's, the explicit publisher path's, and the claim verifier's among them. Every family this grouping names places each concrete's settings admission and the family's declaration lookup, selection admission, and composition admission ahead of its factory at its eventual ticket positions, its member and value identifiers provider-owned with total spellings and declared wire codes, and every consumer reaching it through the admission surface it uses rather than the factory alone.

### Durable jobs and configuration

**Purpose.** The durable mechanisms every daemon workflow runs on: telemetry, the job engine, the configuration registry, platform paths, the IPC family, and the single-instance lock.

**Scope.** `telemetry/local_metrics` over the storage family's admitted selection and factory with the RO-03 record, authoring the exporter interface, the declaration, and the family's mock as its producers and creating the `adapters/telemetry` crate, with its own settings admission; `telemetry/tracing`, the family-owned tracing setup carrying per-request correlation identifiers; `telemetry/opentelemetry`, the OpenTelemetry exporter concrete over the same interface, with its own settings admission; the family's declaration lookup, selection admission, and composition admission ahead of `telemetry/factory`, with a branch per member; `workflows/jobs` over the storage family's admitted selection and factory with checkpoints, idempotency keys, restart, and cancellation; `workflows/config`, the versioned configuration registry over the storage and encoding families' admitted selections and factories, every setting scope carried as the catalogue's typed identifier; `platform-paths/linux`, authoring the application-directory interface, `platform-paths/macos`, `platform-paths/windows`, each concrete's settings admission and the family's three admission functions ahead of `platform-paths/factory` for the store-root defaults, the platform identifier the family's own; the family-owned `ipc/framing` and `ipc/principals`, `ipc/unix_socket` authoring the IPC transport interface, `ipc/named_pipe`, `ipc/server` with the control-principal rule and the local-presence confirmation, each concrete's settings admission and the family's three admission functions ahead of `ipc/factory`, the principal and presence-capability identifiers the family's own, with the request-class identifier and routing owned by `apps/daemon` and the IPC server naming no request class; `lifecycle/lock`, the single-instance lock as a family-owned function creating the `adapters/lifecycle` crate, the family's members, their settings admission, the family's three admission functions, and factory following in the installation coordinator.

**Entry.** Workspace and discipline bootstrap; the domain model; the storage family from the swarm milestone. Independent of the contract milestones.

**Exit.** XA-03 termination at every persisted transition with exactly-once effects; IC-04 for the configuration registry; XA-02 other local users gain nothing, a lifecycle script running as the installing user obtains resolution only, and every local-presence operation waits on the daemon-owned interactive confirmation, through the IPC server; XA-10 backoff within declared ceilings; RO-04 correlation across this milestone's modules.

**Deliverables.** The telemetry, platform-paths, and IPC crates, each family's member identifier, its concretes' settings admission, and its three family-owned functions among them; the lifecycle crate with its lock; the job engine and the configuration registry.

**Owner.** Daemon implementer.

**Unblocks.** Every daemon workflow; the installation coordinator.

### Identity and custody

**Purpose.** The principal, its keys, and where they live.

**Scope.** The family-owned `custody/holder_seed`, deriving the control branch, holding the handshake key, and the envelope branch, holding the envelope secrets, on a root only through the KDF family, creating the `adapters/custody` crate; `custody/device_key`, generation on every device through the signature and random families; `custody/local_keystore` over `keyring` with a wrapping key and version-headed encrypted blobs, the only module touching the OS credential store, authoring `IKeyCustodyAdapter`, the versioned custody-role identifier declaring root and reader as the family's real members with read-delegate and signer held outside the declared set until real members provide them, the custody-adapter-version identifier, and the family's mock as its producers, carrying its own settings admission; `custody/upgrade`, the in-place migration job between custody concretes through the interface, verified against the on-chain binding and envelope-key registration; the family's declaration lookup, selection admission, and composition admission, then `custody/factory`; `wallet/eip1193`, authoring the signing-only wallet interface and carrying its own settings admission, `wallet/walletconnect` with its own settings admission, the family's declaration lookup, selection admission, and composition admission, and `wallet/factory`, each wallet selection carried as the family's member identifier; `identity/publisher_authority`, authoring `IIdentityAdapter`, the binding schema with the DID Document types and anchor hash, and the proof-class identifier declaration, carrying its own settings admission, `identity/escrow` with its own settings admission, the family's declaration lookup, selection admission, and composition admission, and `identity/factory`; `workflows/identity` create or import, relayer-paid binding and identity contract account creation through the chain and submission families' admitted selections and factories, envelope-key registration with proofs of possession, device pairing by QR code or short authentication string, signer admission, re-scoping, and revocation on the contract account, role delegations under the handshake key, envelope secrets delivered to a reader over the paired channel, signing-only wallet connections through the wallet family's admitted selection and factory; the persistent credential store per entitlement; the secret region types.

**Entry.** Durable jobs and configuration; signature services; the chain and submission families, for binding and registration on Base Sepolia; the relayer or paymaster milestone, for the submission family's admitted selection and factory.

**Exit.** IW-01 through IW-07: distinct scheme identifiers composed and surviving restart; capability-identifier combinations failing closed; one identity, one binding, one registration across restart and reinstall; no decrypted credential or key in any custody surface; a reader device paired to a root on a second device profile reading alone with the seed never leaving the root, then revoked and purging its envelopes and envelope secrets at its next state view; recovery of the root on a fresh profile from the seed; an in-place custody upgrade preserving the principal, and a tampered rewrap rolled back; wallets signing only; envelope keys never wallet keys.

**Deliverables.** The custody, wallet, and identity crates, each family's member and role identifiers, its concretes' settings admission, and its three family-owned functions among them; the identity workflow.

**Owner.** Daemon implementer with the cryptography implementer for derivation.

**Unblocks.** Entitlement acquisition; the installation coordinator; the publisher engine.

### Plaintext CAS and package host

**Purpose.** The verified plaintext store and the local registry surface every install is served through.

**Scope.** `cas/filesystem` with the content-hash layout of verified tarballs under the repointable root, atomic commit, eviction warnings, its index through the storage family's admitted selection and factory, and nothing linking into the store, authoring the store interface with commit, serve, pin, quota, eviction, and reuse metrics as its producers and carrying its own settings admission; the family's declaration lookup, selection admission, and composition admission, then `cas/factory`; `package-host/npm` serving tarballs on the loopback port with canonical tarball URLs for lockfile portability and packuments assembled from the npm member's alias records, the decrypted tarballs' own `package.json`, and each entry's signed statement, with no metadata store and no metadata capture, authoring the npm name type, `IPackageHostAdapter`, the declared namespace, and the name-to-root alias lookup as its producers and carrying its own settings admission, the lookup selecting an alias entry only on the client's own evidence, a signed statement checked against the lockfile `integrity` or the source or an unsigned entry decrypted and compared, and failing closed on an alias without evidence, then the family's declaration lookup, selection admission, and composition admission, then `package-host/factory`; store migration jobs for ST-02. Both families are adapters beneath their factories and name no workflow.

**Entry.** Durable jobs and configuration; pairing adapters and key derivation, for the hashing and encoding families.

**Exit.** PR-02 name-to-root resolution on the client's evidence, a competing alias resolved by the lockfile `integrity` or the source, an alias without evidence refused naming the roots; PR-04 under interruption, races, and corruption; PR-05 lifecycles separate; PR-09 packuments assembled from alias records and decrypted tarballs with no metadata store, every ingested version the client can decrypt served with upstream unavailable and dist-tags best effort; ST-02 migration with interruption.

**Deliverables.** The CAS and package-host crates with their npm and filesystem members, each family's settings admission and three family-owned functions among them.

**Owner.** Daemon implementer.

**Unblocks.** First Finder engine; the resolution orchestrator.

### First Finder engine

**Purpose.** Absent packages enter the swarm without slowing the install that fetched them.

**Scope.** `ingest/npm` with as-is fetch, registry-signature attestation validation against the registry's published keys or recorded absence, written as the alias entry registered with the root and carrying nothing the bytes contain, and public-availability eligibility, authoring `IIngestSourceAdapter`, the attestation-presence and ingest-source identifier declarations, with the same namespace the package host declares, as its producers, and carrying its own settings admission; the family's declaration lookup, selection admission, and composition admission, then `ingest/factory`; `workflows/first_finder` foreground serve and background durable job: deployment identity under state lock, master scalar and parameter set through the KEM family's admitted selection and factory, a piece-group key and capsule randomness per group and the IV through the randomness family's admitted selection and factory, encryption per group through the cipher family's admitted selection and factory, one sidecar per live set through `workflows/sidecar/build` and the hash-card, registration race on the root with loser destruction, escrow custody of the master scalar, seed handoff through the seed-host family's admitted selection and factory, and the finder's grant of the asset's first entitlement to itself.

**Entry.** Plaintext CAS and package host; swarm transport and seed host; registry contracts; the chain and submission families; the relayer or paymaster milestone, for the submission family's admitted selection and factory; identity and custody; payload cipher and sidecar layer; the credential KEM, envelope, and delivery proof families for the self-grant.

**Exit.** FF-01 through FF-07 and AS-09, AS-10, AS-12: verified bytes served with every background dependency unavailable, forged and mismatched attestations refused and a missing one recorded; single completion after termination at every checkpoint; production randomness validated; race with one winner and clean losers, and a later escrow deployment of an existing asset admitted; the master scalar retained under custody; the self-grant recorded once.

**Deliverables.** The ingest crate and First Finder workflow.

**Owner.** Daemon implementer.

**Unblocks.** Credential delivery; the resolution orchestrator; explicit publishing.

### Credential delivery and per-attempt authorization

**Purpose.** Close the read path: nothing requested from anyone at read time, and nothing decrypted without a current view.

**Scope.** `payment/native`, authoring `IPaymentAssetAdapter`, the payment-asset identifier, the declaration of the assets a member settles, and the family's mock as its producers and creating the `adapters/payment` crate, carrying its own settings admission; the family's declaration lookup, selection admission, and composition admission, then `payment/factory`; `workflows/request`, one batched sponsored request per install for every unheld ledger-known asset, queued until submittable, with pickup of grants from chain events on any later run, its states carried as typed request-state identifiers; `workflows/prefetch`, ciphertext and sidecar while a request is pending, under the seeding settings and quota, pinned, seeded, and matched to the granted set; `workflows/acquire` with relayer-paid mint, escrow grant, and funded purchase with a lock naming its payment-asset identifier, reaching payment only through the payment family's admitted selection and factory; `workflows/grant`, the grant service under the asset scope, following the credential engine and the request job, fulfilling open requests for held escrow assets as it seeds, requesters present or absent, its outcomes carried as typed grant-outcome identifiers; the credential engine decrypting envelopes into the secret region through the envelope family's admitted selection and factory, the validity check through the KEM family's admitted selection and factory, in-memory rerandomization, zeroization; `workflows/attempt` with `AttemptContext`, freshness clocks, the wallet-control assertion by a device key the identity's contract account admits in the same state view, the attempt's typed three-state outcome, batch form; `workflows/decrypt` with decapsulation through the KEM family's admitted selection and factory, unwrap through `workflows/sidecar/unwrap`, AES-CTR addressing through the cipher family's admitted selection and factory, Bao verification through the hashing family's admitted selection and factory, CAS commit, streaming; the interval-end watcher; the deployment gate; sale delivery from a fresh decryption with the transfer proof through the proof family's admitted selection and factory.

**Entry.** Identity and custody; the chain and submission families; the KEM, envelope, and proof families; payload cipher and sidecar layer; registry contracts; First Finder engine; the relayer or paymaster milestone, for the submission family's admitted selection and factory.

**Exit.** EC-01 through EC-08 and AS-07, AS-08 with the ingest source disabled, AS-19; CD-01, CD-02, CD-04, CD-08 and AS-14; CD-05's automatic grant service with the First Finder offline, a requester offline when its grant is authored and loading it on its next run, and a swarm of only non-holder seeders reporting no grant author truthfully, AS-26; PR-10 and PR-11 through AS-29, the first run ending independent with every branch exercised, held, absent, dead-deployment, explicit-publisher, and priced assets; the delivery scenario AS-13 through the daemon's workflows rather than the harness; the grant-request and grant-pickup integration points crossed; storage and crash artifacts free of decrypt-capable material; destruction at the `HARD` settlement-tier identifier and a reorganized transfer leaving the seller able to read; a reader device denied and purging at its next state view after its signer is revoked.

**Deliverables.** The acquisition, credential, grant, attempt, decryption, and interval workflows.

**Owner.** Daemon implementer with the cryptography implementer.

**Unblocks.** The resolution orchestrator; claim; the transaction flow proof.

### Resolution orchestrator

**Purpose.** The benefit a developer feels: every install served from whichever source delivers within the configured source deadline, never slower than the registry alone by more than the hedge delay.

**Scope.** `workflows/resolve` trying sources in order and hedged under the hedge delay, source deadline, and grant wait its params carry: the CAS, ciphertext under a held credential through the attempt engine and the decryption pipeline, the seed host and the swarm, upstream through the ingest family's admitted selection and factory for any identity without a credential, and a bounded wait on a requested grant when upstream is down, with resolution-path and pending-reason metrics carried as typed identifiers; the upstream plaintext-root check against the canonical record through the hashing family's admitted selection and factory; `workflows/health` with its independence-status outcomes carried as typed identifiers per asset and per project.

**Entry.** Plaintext CAS and package host; swarm transport and seed host; First Finder engine; credential delivery and per-attempt authorization.

**Exit.** PR-01 parity with upstream across representative projects; PR-03 each source-identifier choice and its typed reason recorded with and without a held credential and with upstream available and unavailable, the hedge start, the bounded wait, and the pending failure asserted; PR-06 and AS-06 plaintext hit with everything else unreachable; PR-08 the foreground install's latency unchanged with every background stage stalled; PR-09 a lockfile committed on one machine installing on another with a different host port; PR-12 independence status per asset and per project; ST-02 prior projects still installing after a store migration.

**Deliverables.** The resolution and health workflows.

**Owner.** Daemon implementer.

**Unblocks.** The daemon composition.

### Daemon composition

**Purpose.** The daemon's and the installer's composition roots, authored once every module they compose exists.

**Scope.** `workflows/compose`, the composition resolver that works back through the declarations of every family the daemon and the installer compose, completing each family's composition admission before any construction or side effect, failing closed; `workflows/settings` with the catalogue of every module's settings, each setting's scope carried as the catalogue's typed setting-scope identifier, including its presence-required marks and the hedge delay, source deadline, and grant wait, validation, migration jobs, backup and restore, export and import as a versioned TOML document rendering canonical spellings only at that boundary; `apps/daemon`, the daemon binary and its composition root, handing every module its admitted settings as params; `apps/installer`, the installer binary; per-platform packages through `cargo-dist`.

**Entry.** Resolution orchestrator; identity and custody; installation coordinator and platform adapters; explicit publisher path and issuance policy; claim verifier and escrow claim.

**Exit.** IC-05 with no side effects from invalid compositions; ST-01 through ST-04 and ST-08 through the IPC; every presence-required setting waiting on the daemon-owned interactive confirmation; AS-03, AS-04, AS-05, AS-18, and AS-30 on every supported platform through the installer binary and the packaged daemon; the daemon's footprint limits and cold start; RO-04 correlation across the daemon.

**Deliverables.** The composition resolver and the settings service; the daemon and installer binaries; per-platform packages.

**Owner.** Daemon implementer with the platform implementer.

**Unblocks.** The demonstrable milestone; CLI and desktop application; observability reconciliation; demonstration harness; the transaction flow proof.

### Demonstrable milestone: an install resolves against the swarm

**Purpose.** The earliest point the north star and install wall-clock can be observed on the dogfood population, and the first thing the project can show.

**Scope.** No new subsystem. A second clean identity on a second machine profile installs a pinned closure the first ingested, at the registry's speed with the requests registered and the ciphertext prefetched, and then, with the registry unavailable, through the packaged daemon and package host over the swarm, on a grant from the first identity fulfilled while the second was offline.

**Entry.** Daemon composition; swarm transport and seed host.

**Exit.** FF-08 and AS-11, AS-24, and AS-29 in substance; the north star, cross-project reuse, and the fraction of the closure independent after the first run measured on the dogfood population; whole-command elapsed time and authorization fraction reported against baseline, with any adjustment of the configured defaults the data supports.

**Deliverables.** The dogfood measurement record.

**Owner.** Project lead.

**Unblocks.** Confidence; nothing technical.

## Onboarding shells and services grouping

The relayer or paymaster milestone is re-mapped to tickets with the daemon grouping, ahead of the identity and custody milestone, which consumes its submission family's admitted selection and factory. The installation coordinator, the explicit publisher path, and the claim verifier are re-mapped ahead of the daemon composition, which follows them. The CLI and desktop application, observability reconciliation, and the demonstration harness follow the daemon composition, and the extension and npm bootstrap follow the installation coordinator.

### Installation coordinator and platform adapters

**Purpose.** Either entry point produces one identical working system on a clean machine after only permitted consent.

**Scope.** `workflows/install` durable checkpointed plan; `lifecycle/systemd`, authoring the service-lifecycle interface, `lifecycle/launchd`, `lifecycle/windows_service`, `lifecycle/scheduled_task`, each member carrying its own settings admission and the family's declaration lookup, selection admission, and composition admission ahead of `lifecycle/factory`, each lifecycle selection carried as a typed member identifier, in the crate the lock in durable jobs and configuration created; `artifact-verifier/sigstore`, authoring the signed-artifact verification interface, `artifact-verifier/authenticode`, `artifact-verifier/apple_notarization`, each member's settings admission and the family's declaration lookup, selection admission, and composition admission ahead of `artifact-verifier/factory`, for artifact download and verification before execution; `redirect/npm` across the user, project, workspace, proxy, and custom-registry scope identifiers, authoring the detect, redirect, back-up, and restore interface and carrying its own settings admission, then the family's declaration lookup, selection admission, and composition admission, then `redirect/factory`; backup and rollback; idempotent reinstall; repair; signed reversible update; uninstall with preservation; the initial settings and consent items its params carry offered under their typed consent-item identifiers, the path items, the request and prefetch items with the first-run cost disclosure, and the cache-only mode among them; the reserved account-link step shown as unavailable; the first-run disclosure. The installer binary follows in the daemon composition.

**Entry.** Durable jobs and configuration; identity and custody; the signing identities enrolled at bootstrap.

**Exit.** SI-03 through SI-18 and SI-20, IC-01 through IC-03, IC-06, IC-07, and ST-07, each through the install workflow on every supported platform, each platform member's lifecycle and verification selections admitted through their families' admission functions and resolved through their factories before any installation effect; the consent trace clean under SI-19.

**Deliverables.** The install workflow; the lifecycle members, their settings admission, the family's functions, and factory; the artifact-verifier and redirect crates with each family's admission functions.

**Owner.** Platform implementer.

**Unblocks.** The daemon composition; the extension and npm bootstrap.

### Relayer or paymaster

**Purpose.** A first-run user never acquires gas to install a free package.

**Scope.** `apps/relayer` with the chosen ERC-4337 bundler and paymaster provider's admitted member, authoring the bundler-provider interface, and `relayer/bundler/factory`, the provider evaluated here; the sponsorship mechanism for the identity contract account chosen here under LC-13's signed-intent rule, with `submission/paymaster` and `submission/relayer` authored as the chosen mechanism requires and depending on no module of `apps/relayer`, since the relayer service depends on the submission family, the relayer service's request and response types authored in `submission/relayer`'s node as their first consumer, each member carrying its own settings admission; then the family's declaration lookup, selection admission, and composition admission, then `submission/factory`, authored once with a branch per submission member; then the relayer service, with sponsorship overhead measured; sponsor endpoints for the one binding, device admission and revocation, one batched request per install, free mints, and fulfilling grants; admission policy with a global per-window budget and maximum sponsored liability first and per-identity rate limits sized to a realistic closure as one layer; explicit typed failure outcomes for exhaustion and denial; cost and budget reporting through the chain family's fee description; the grant pool sized from harness and dogfood gas.

**Entry.** The chain and submission families; registry contracts. It precedes the identity and custody milestone, the First Finder engine, and credential delivery, each of which consumes the submission family's admitted selection and factory.

**Exit.** LC-13's self-funded call path and the chosen sponsored path through the submission family's admitted selection and factory; RO-01 clean free onboarding, a device admission and a revocation sponsored, a batched request sponsored and its fulfilling grants sponsored, subsidy exhausted and denied with actionable recovery as typed outcomes, requests queued locally, and no false authorization; LC-10 and AS-27 a flood of fresh identities never spending beyond the configured budget and exhaustion reported truthfully; RO-02 paid acquisition refused without funds in the payment-asset identifier the lock names, and never sponsored.

**Deliverables.** The relayer service with its bundler factory and the chosen provider's admitted member, the sponsored submission members with their settings admission and the family's functions, and its operating runbook.

**Owner.** Relayer operator.

**Unblocks.** Identity and custody; First Finder engine; credential delivery; free onboarding end to end; the transaction flow proof.

### CLI and desktop application

**Purpose.** The two control surfaces the project owns outright, over the daemon's IPC.

**Scope.** `apps/cli` with `clap`: consent, settings, diagnostics, package-host control, identity, jobs, repair, uninstall; `apps/desktop` on Tauri 2: configuration, identity and custody workflows including device pairing, admission, and revocation, the custody upgrade, and root recovery, adapter selection through each family's member identifier, jobs, the seeding archive, the settings catalogue, diagnostics, repair, recovery.

**Entry.** Installation coordinator; daemon composition, for the settings catalogue.

**Exit.** IC-08 and RO-05 identical facts across surfaces; PR-12 independence shown identically on every surface; ST-05 surface parity for every catalogue entry; SW-07 the archive view matching the host; SI-16 repair from each surface; IC-10 accessibility baselines.

**Deliverables.** The CLI and desktop packages.

**Owner.** Platform implementer.

**Unblocks.** Acceptance scenarios that exercise every surface.

### Visual Studio Code extension and npm bootstrap package

**Purpose.** The two onboarding paths the specification names, as thin shells.

**Scope.** `shells/vscode-extension` in TypeScript over the IPC client, bundling or fetching the signed installer, status and consent views, no protocol logic; `shells/npm-bootstrap` as a `postinstall`-free package whose `bin` invokes the signed installer.

**Entry.** Installation coordinator.

**Exit.** SI-01 and AS-01; SI-02 and AS-02; both paths converging on one postcondition, IC-01; each running the signed installer binary the daemon composition builds, a runtime input.

**Deliverables.** The extension and the npm package, published to their hosts' test channels.

**Owner.** Platform implementer.

**Unblocks.** Acceptance.

### Explicit publisher path and issuance policy

**Purpose.** The project publishes its own package, seeding the swarm with its first content and supplying the only assets that may carry a price.

**Scope.** `workflows/publish` with seed-derived parameter sets and capsule randomness through the KDF family's admitted selection and factory, `publisher-proof/provenance`, authoring `IPublisherProofAdapter` and the proof-class and strength identifier declarations, `publisher-proof/maintainer_oauth`, each member carrying its own settings admission and the family's declaration lookup, selection admission, and composition admission ahead of `publisher-proof/factory`, resolving the declared members in strength order, the proof-class identifier recorded on chain, dependency-closure ingestion as First Finder, the issuance policy service serving mints automatically under configuration with per-request approval as a declared capability; the publisher seed under custody.

**Entry.** First Finder engine; identity and custody; registry contracts.

**Exit.** PC-01, PC-03 replay and redirection rejected, FF-09 and AS-22, AS-23; a second identity consumes the published package; the issuance policy serves a mint with no person present.

**Deliverables.** The publish workflow and the publisher-proof crate with its members' settings admission and the family's functions; ChainTorrent's own package published on Base Sepolia.

**Owner.** Daemon implementer.

**Unblocks.** The daemon composition; the transaction flow proof; the project seed host's content; the claim verifier's inputs.

### Claim verifier and escrow claim

**Purpose.** A maintainer takes complete ownership of escrowed records without the First Finder and without any stored maintainer binding.

**Scope.** `apps/claim-verifier` running provenance-then-OAuth verification through the publisher-proof family's admitted selection and factory, establishing the claim set from upstream metadata at verification time, signing vouchers bound to claimant, set, contract, chain, nonce, and expiry under a custody-held key of a declared signature-scheme identifier registered on chain as a scheme identifier and a key commitment with rotation and revocation; `claim-verifier/attestor` as the daemon's client of the attestor service, authoring `IClaimVerifierAdapter`, the voucher types, and the declaration of the signature-scheme identifier its vouchers carry, and carrying its own settings admission, then the family's declaration lookup, selection admission, and composition admission, then `claim-verifier/factory`; the escrow claim contract surface in the registry; `workflows/claim` in the daemon; claimant parameter-set registration, optional encrypted handover with compatibility proof, successor deployments with a sidecar per live set, sidecar addition to live escrow deployments from an escrow-era credential, voluntary migration and retirement.

**Entry.** Explicit publisher path; registry contracts; credential delivery.

**Exit.** PC-04 through PC-07 and CD-06; AS-16 claim with the First Finder absent, successor readable under both sets, the escrow deployment opening under the claimant's set after sidecar addition, one holder migrated; AS-27's revoked-key voucher rejected; PC-07's vouchers verified through the signature adapter the registry binds for the key's scheme identifier, a key not matching its commitment refused.

**Deliverables.** The verifier service; the claim-verifier crate with its member's settings admission and the family's functions; the claim workflow; the contract surface deployed.

**Owner.** Claim verifier operator with the contract implementer.

**Unblocks.** The daemon composition; acceptance.

### Project seed host, site, and WebAssembly demonstration

**Purpose.** A seed of the core closure from day one and an education tier a prospective developer can try with nothing to sign up for.

**Scope.** The packaged daemon the daemon composition builds, a runtime input, configured as a seed host for persistence, seeded by the dogfood publication, and holding an entitlement and credential for every asset it seeds; the static site; `apps/wasm-demo` built with `wasm-bindgen` from the domain, hashing, pairing, KEM, and cipher crates, running against the sample deployment generated at the payload cipher milestone; the download links; the statement that seeding and the CAS do not run in a browser.

**Entry.** Explicit publisher path; swarm transport and seed host, for the owned member; payload cipher and sidecar layer, for the sample deployment.

**Exit.** The core closure retrievable from the seed host after publication, FF-09; SW-08, a new identity obtaining a grant for a core-closure asset from the seed host with every other holder offline; the demonstration performing real resolution, verification, and decapsulation with no network access on the read path; the protocol resolving with the seed host unreachable.

**Deliverables.** The seed host deployment; the site; the demonstration bundle.

**Owner.** Project lead with the daemon implementer.

**Unblocks.** Cold start; the prospective-developer audience.

### Observability reconciliation

**Purpose.** Every metric the economics depend on, reconciled against induced activity, with no secret anywhere.

**Scope.** Completion of the RO-03 record across every emitting subsystem, each metrics-category field a typed identifier; RO-04 correlation across concurrent installs; RO-06 attempt latency and install wall-clock measured and reported against the configured values in force, proven by injected delay reflected accurately; the telemetry scan; secret-free failure messages under XA-04.

**Entry.** The daemon composition, which follows every daemon module that emits; every service milestone, since each emits.

**Exit.** RO-03 through RO-06 and XA-04; AS-20 in rehearsal.

**Deliverables.** The reconciled metrics schema and scan.

**Owner.** Observability implementer.

**Unblocks.** The acceptance run.

### Demonstration harness

**Purpose.** Reproducible participants, wallets, chain state, and failures, without which the completion boundary cannot be met.

**Scope.** `apps/harness-demo`: spawned daemons under distinct identities and machine profiles, Anvil and Base Sepolia chain state, wallets funded and unfunded, fault injection at process, network, disk, and chain, the race and reorganization drivers, the deliberately incompatible adapter declarations the incompatibility scenario rejects — fixtures outside every family's real declared set — refused by the families' composition admissions and by `workflows/compose`, the clean-machine runner integration for CI.

**Entry.** Daemon composition, whose composition resolver the incompatibility scenario exercises.

**Exit.** XA-07 every requirement's proof class mapped to a facility; XA-03 termination at every persisted transition driven by the harness; every scenario from AS-06 onward runnable unattended.

**Deliverables.** The harness and its scenario definitions.

**Owner.** Test infrastructure implementer.

**Unblocks.** The acceptance run.

## Acceptance and release grouping

### Transaction flow proof on the Base mainnet pilot

**Purpose.** Prove a nonzero transaction settles, on the one asset class where pricing is legitimate, while the entitlement graph is two identities.

**Scope.** Contract suite deployed to Base mainnet; ChainTorrent's package published and priced trivially above gas under the issuance policy; a funded buyer acquires in the payment-asset identifier the lock names, Base's native-asset member through the bound payment-asset adapter; the holder resells; the settlement boundary asserted; every guardrail live.

**Entry.** Explicit publisher path; credential delivery; relayer; daemon composition; registry contracts on mainnet; external cryptographic review dispositioned, which gates any priced deployment; legal review of priced entitlements and the paymaster.

**Exit.** LC-03 First Finder pricing refused; LC-05 and CD-02 the seller's proof verified and payment released in the lock's payment-asset identifier through the bound payment-asset adapter in one transaction; RO-02 real funds required; AS-15 the buyer decrypting at the declared settlement-tier identifier and the seller destroying at the `HARD` identifier.

**Stop criterion.** No priced deployment before review and legal closure; a composition or verifier finding not dispositioned under its typed disposition value halts here.

**Deliverables.** The mainnet deployment record; the proof's settlement records.

**Owner.** Project lead with the contract implementer and relayer operator.

**Unblocks.** The acceptance run's priced scenario.

### External review and legal prerequisites closed

**Purpose.** The two release conditions engineering does not control, started at the harness and closed here.

**Scope.** Cryptographic review findings on the composition claims and the verifier tracked to their typed dispositions; license text adopted and the content-terms field populated for the project's packages, LI-01; the eligibility principle and residual restated in MVP Scope; regulatory review of priced entitlements and the paymaster; export constraints on the packaged binary confirmed; code-signing certificates in hand.

**Entry.** Harness measurement and report, for review inputs; nothing else technical.

**Exit.** Every finding dispositioned under its typed disposition value and recorded in release evidence; every legal item recorded as a release-evidence document.

**Deliverables.** Release-evidence documents.

**Owner.** Project lead.

**Unblocks.** Release.

### Acceptance run

**Purpose.** The completion boundary, met or not.

**Scope.** Every acceptance scenario through the packaged applications on clean machines on every supported platform member, driven by the demonstration harness against Base Sepolia, the priced scenario against the Base mainnet pilot, with every metric reconciled against induced activity and the telemetry scanned; no test-only bypass of onboarding, the package host, capability resolution, or the deployed verifier.

**Entry.** Every milestone above.

**Exit.** Every acceptance scenario passes; every guardrail holds; the requirement-to-proof reconciliation is complete.

**Stop criterion.** Any guardrail breach blocks release, and no guardrail may be waived by adjusting the metric.

**Deliverables.** The acceptance record.

**Owner.** Project lead.

**Unblocks.** Release.

### Dogfood baseline and release

**Purpose.** Ship, with the numbers that make the next decision possible.

**Scope.** The dogfood baseline for the north star, primary KPIs, and leading indicators recorded on the Base mainnet pilot alongside the harness measurements, the mainnet re-take of the L1 data fee, the chosen piece-group size, curve identifier, and attempt-rule parameters, each closed-set value carried as its provider-owned identifier and rendered canonically only in the evidence; signed packages published; the extension and npm package published; the site live with the demonstration; the reporting cadence set; the next planning cycle's re-map of deferred items begun.

**Entry.** Acceptance run; external review and legal prerequisites closed; transaction flow proof.

**Exit.** Release evidence complete under the Application Requirements' completion boundary; both onboarding paths reaching the same postcondition from published artifacts.

**Deliverables.** The release and its evidence.

**Owner.** Project lead.

**Unblocks.** The adopting population.

# Iteration Semantics

**Resolution decays and re-maps.** The foundation and harness groupings and the hashing, signature, and swarm milestones are at ticket resolution. When the `contracts/evm/deploy` node commits, the remaining protocol core milestones are re-mapped from sprints to tickets using what the harness taught and its measured throughput, the daemon grouping from epics to sprints, and so on outward; this document and the dependency map are revised in place.

**Nodes are authored from tickets through the ordinary path.** A ticket names a source file's role, dependencies, and proof; the workplan author writes the node in the fixed element order, resolving every path and symbol so the implementer instantiates rather than invents; the implementer builds it under the repository's process topics. A family's first node, its first concrete or a family-owned function preceding it, authors the family's interface, its member and value identifiers, and declaration as its producers, a concrete adapter is one node whose operations are its methods and whose settings admission follows it, each family-owned admission function is a node following the members it covers, the factory node is authored once after the family's last member and admission function, and a consumer node depends on the admission surface it uses — a concrete's settings admission, a family's declaration lookup, selection admission, or composition admission — or on the factory's surface, and never on a concrete.

**Only code orders nodes.** A node depends on another only when its file needs the other's file in order to be written; generated output, a deployed contract, and a measured value are runtime inputs, sequenced in the dependency map's Runtime sequence or stated where a milestone's exit runs them, and none orders a node or a milestone. Each node's file is written once, complete, at its position.

**A milestone closes on its commit.** The last node in a milestone's chain carries the integration test across the chain and the commit; the milestone's exit rows are the requirements that test proves.

**Discovery revises the map and the unbuilt nodes.** An implementer who finds that a node needs a second file, a missing producer, or a decision the node did not make reports and halts; the author revises the map and reorders or rewrites the unbuilt nodes it affects, so every file is still written once, complete, and the milestone's scope is restated in full, never as a delta.

**Stop criteria are evaluated where named.** At the post-harness re-map, the acceptance run, and review closure; each halts the plan at that point when met.

**Parallelism is by dependency only.** Independent milestones may run together when the team allows; the map's edges, not a calendar, decide.
# Features Context

The in-scope features of the [feature spec](feature-spec.md) map onto milestones as follows. Cryptographic services and validation harness: the harness grouping and the hashing and payload cipher milestones. Adapter composition: the daemon composition. Entitlement ledger and settlement: registry contracts and the chain and submission families. Swarm, discovery, and seed hosting: swarm transport and seed host, and the project seed host. Package serving and CAS: plaintext CAS and package host, and the resolution orchestrator. Encrypted content consumption: credential delivery and per-attempt authorization. First Finder bootstrap: First Finder engine. Credential delivery: credential delivery and per-attempt authorization, and claim. Identity, wallet, and custody: identity and custody. Settings and configuration: durable jobs and configuration for the configuration registry and the daemon composition for the settings service, with surfaces in CLI and desktop. Self-installing onboarding: installation coordinator, the daemon composition for the installer binary, the extension and npm bootstrap. Relaying: relayer. Explicit publishing and escrow claim: explicit publisher path, claim verifier and escrow claim. Project seed host and site: its milestone. Observability: observability reconciliation. Test facilities and demonstration harness: demonstration harness and the acceptance run. Transaction flow proof: its milestone. The deferred features and the travelling-developer story constrain choices in identity and custody, the IPC in durable jobs and configuration, and the publisher path's issuance policy, and consume no milestone.

# Feasibility Insights

From the [feasibility assessment](technical-feasibility.md): technical feasibility high, delivery feasibility undetermined until the harness measurement and report calibrates throughput, which is why that milestone carries the re-map and the funding stop criterion. Platform breadth is the largest delivery risk and is concentrated in the installation coordinator milestone, whose scope is bounded by the decided platform-member matrix. The demonstration harness is load-bearing and is its own milestone. The on-chain verifier, in the MVP suite's declared source language, has its constants and vectors generated from the Rust reference in the delivery-proof milestone. Transfer is proven last, in the transaction flow proof, and its external gates start at the harness so they do not arrive at the end.

# Non-Functional Alignment

Each non-functional requirement of the [review](non-functional-requirements.md) is proven inside the milestone that owns its subsystem: security rows in the harness, contracts, and credential-delivery milestones; performance rows at the harness measurement and report, the demonstrable milestone, and observability reconciliation, each reported against baseline; reliability rows in durable jobs and configuration, the installation coordinator, and the swarm milestones; maintainability rows in the workspace bootstrap and the tickets that own each type; compliance rows in external review and legal prerequisites. The requirements the review added, XA-08, XA-09, XA-10, IC-09, IC-10, RO-07, RO-08, and LI-02, are carried where they land: signing-key custody in the bootstrap and installer; the package-host threat model in the IPC of durable jobs and configuration; footprint limits and cold-start in the daemon composition; backoff bounds in the chain and swarm adapters; the suite compatibility statement at release; regulatory and export review in legal prerequisites; accessibility in CLI and desktop; the first-run cost disclosure in the installer.

# Architecture Summary

As the [system architecture](system-architecture.md) states: one canonical ciphertext per deployment and one sidecar per live parameter set, each its own object, on the swarm, entitlements on Base, credentials delivered inside settlement and verified by proof, per-attempt local authorization, and a daemon serving npm's protocol from whichever of cache, swarm, and registry delivers within the configured source deadline, never slower than the registry alone by more than the hedge delay, leaving a first run independent, with nothing on the read path.

# Services

The daemon, relayer, claim verifier, seed host and site, validation harness, and demonstration harness, each built in the milestone named above and each bounded by what it must never become.

# Components

The rings and the adapter table of the system architecture; the subsystem register of the technical requirements assigns each to its crate and, through this document, to its milestone.

# Dependency Resolution

Owner-admitted selections, declaration lookup, composition admission, suite binding, fail closed, and the on-chain factory, built in the daemon composition and the adapter registry and factory and proven under IC-05, SI-10, and AS-17 in the acceptance run.

# Component Details

The daemon's subsystems, their ownership, trust boundaries, process model, and storage ownership are in the system architecture's daemon breakout; their APIs, schemas, and file tree are in the technical requirements. This document adds only their milestone assignment, which the Milestones section states per scope line.

# Integration Requirements

The integration points of the dependency map are each crossed by the integration test of the milestone that first joins the two sides: family admission in the harness for the cryptographic families and in the daemon composition for the whole composition; verifier parity and precompile encoding in the delivery-proof milestone; the decapsulation-to-cipher seam and sidecar commitment in payload cipher and sidecar layer; the discovery engine in the swarm milestone; the store in the swarm milestone for the ciphertext index, in durable jobs and configuration for the job engine, the configuration registry, and the local metrics, and in plaintext CAS and package host for the CAS index; envelope-in-settlement and the signed intent in the relayer or paymaster milestone; attestation verification on chain in the First Finder engine; attempt context in the chain and submission families for the quorum view and in credential delivery for the attempt; resolution order in the resolution orchestrator; the package-manager protocol in plaintext CAS and package host; the persistent credential in credential delivery; device admission in identity and custody for the identity contract account and in credential delivery for the wallet-control assertion; grant request to holder and grant pickup from events in credential delivery; installer-to-daemon, artifact authentication, and shells-to-coordinator in the installer and shells; relayer-to-contracts in the relayer; voucher-to-contract through the bound signature adapter in claim; payment settlement through the bound payment-asset adapter in the registry contracts for the lock and in credential delivery for acquisition through the payment family's admitted selection and factory; the alias lookup in plaintext CAS and package host for the name-to-root resolution and packument assembly and in the registry contracts for the alias records and the root-keyed asset record; the site demonstration in the seed host and site milestone.

# Migration Context

Migrations in the MVP's own lifetime, and the one beyond it. Settings and store roots migrate through checkpointed jobs under ST-02. Configuration schema migrates under IC-04 and SI-17 with reversible updates. Custody blobs migrate in place between custody members under their adapter-version identifiers by the custody family's checkpointed upgrade job, which verifies the re-derived public keys against the chain before retiring the old blob; a change of storage member migrates each store through the storage family's interface with the same checkpointing. A deployment suite changes only by a successor deployment carrying a sidecar per live parameter set, and a parameter set retires only when no live entitlement remains under it. The `librqbit` overlay migrates onto each upstream release by rebase, with the hard-fork trigger ending that track. Beyond the MVP, entitlements and contract state do not move between chain members, no mechanism exists, and one is required before any second chain, including the project's own, carries live entitlements; it is held in the workplan To-Do and is not scheduled here.
