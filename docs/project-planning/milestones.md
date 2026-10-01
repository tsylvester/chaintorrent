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

The milestones, in groupings by delivery role, carry the work from an empty repository to a released MVP. The foundation grouping establishes the workspace and its lint table, the secret type, the randomness family, and the build on Windows, macOS, and Linux. The cryptographic validation harness grouping builds the domain's identifier modules and the derivation context, the encoding family that follows them, and the pairing, key-derivation, hash-to-scalar, credential KEM, envelope, and delivery proof families, each from its first ticket, which carries the family's interface, to its factory, which follows every concrete, together with the chain family's EVM forms concrete and its family-owned forms function, the Solidity verifier, the harness's own generate and verifier families, and the harness's typed configuration and binary, measures them on Base Sepolia against the configured piece-group size and curve defaults, records any adjustment the data supports, and yields the project's throughput calibration. The protocol core grouping builds the hashing, signature, transport, discovery, storage, and seed-host families at ticket resolution beside the harness, without any chain, with `librqbit` as the transport concrete, and the cipher family with the sidecar layer, the EVM contract suite as the chain family's on-chain concrete, and the chain and submission families after the harness report. The daemon grouping builds the process every requirement family meets, closes the read path with credential delivery and per-attempt authorization, and then reaches the demonstrable milestone at which an ordinary `npm install` is served at the registry's speed, registers and prefetches, a second identity resolves against the swarm with the registry down, and the north star, first-run independence, and install wall-clock are observed. The shells and services grouping builds installation on Windows, macOS, and Linux, the control surfaces, the relayer, the publisher path, the claim verifier, the site with its WebAssembly demonstration, observability reconciliation, and the demonstration harness. The acceptance and release grouping runs the transaction flow proof on the Base mainnet pilot, closes external review and legal prerequisites, runs every acceptance scenario on clean machines, records the dogfood baseline, and releases.

Each milestone states its purpose, scope by subsystem and ticket, entry conditions, exit proof by requirement and scenario identifier, deliverables, owner role, and what it unblocks. Resolution follows the dependency map's decay: the foundation and harness groupings and the hashing, signature, and swarm milestones are at ticket resolution; the remaining milestones are re-mapped to tickets as the harness closes and outward from there. The stop criteria from the product requirements sit at the milestones where they can first be evaluated.

# Pipeline Context

This document sits between the product and technical requirements above it and the workplan's nodes below it. The [technical requirements](technical-requirements.md) fix what the system must contain; the [dependency map](dependency-map.md) fixes what depends on what and holds the harness at ticket resolution; this document segments that into milestones with entry and exit conditions; the workplan's Work Breakdown Structure then holds one node per source file, authored through the repository's ordinary path from the tickets a milestone names. The master plan, which the pipeline places before this document, summarizes these milestones into the implementation view; it can be produced from this document without loss.

Two repository rules govern everything below. Nodes and groupings are addressed relationally, never numbered. Each node is one source file with its full support system, authored test-first in the fixed element order, one file per turn, with a commit only at the end of a chain that can be integration-tested.

# Selection Criteria

A milestone is drawn where all of the following hold.

- **It ends at a provable boundary.** Its exit is a set of requirement rows and, where one exists, an acceptance scenario, proven through the boundary the requirement names, never by a unit test where the proof crosses a process, adapter, storage, network, custody, or chain boundary.
- **Its dependencies are complete before it starts.** Entry conditions name the milestones that must have closed; nothing inside a milestone waits on something outside it.
- **It is the smallest span that yields something the next milestone consumes.** A milestone that produces nothing consumable is merged into its consumer; one that produces two independent things is split.
- **It carries a commit.** The last node in a milestone's chain holds the integration test and the commit, per the workplan-structure rule that neither is ever a node of its own.
- **Where a stop criterion can first be evaluated, the milestone names it.** The post-harness re-map, the acceptance run, and review closure each carry one.
- **It has one owner by role.** Roles are those the product requirements and success metrics use.
- **It begins each family it introduces at that family's first ticket and closes the family at its factory.** The first ticket, the family's first concrete or a family-owned function preceding it, authors the family's generic interface, capability declaration, and mock as its producers and creates the crate; each further concrete and family-owned function follows; the factory ticket, which constructs the concretes, follows every concrete, carries the family's integration test across factory and concretes, and carries the milestone's commit where the family closes the milestone's chain; a concrete a later milestone adds revises the factory ticket in place. No ticket consumes a concrete, and the declarations exist from the family's first ticket.

# Shared Infrastructure

Built once in the foundation grouping and consumed by every later milestone.

| Infrastructure | Provides | Built in |
| --- | --- | --- |
| Cargo workspace with rings as crates | A domain crate, a workflows crate, one crate per adapter family holding its `factory` module and its private concretes, apps, each created by the node of the first module that lives in it; glob members; a lint table forbidding `unsafe_code` and denying `unwrap_used`, `expect_used`, `panic`, and `as_conversions` in production code; a ring violation is a compile error | Workspace and discipline bootstrap for the manifest; each crate at its first module |
| Continuous integration on clean ephemeral runners for Windows, macOS, and Linux | `cargo check`, `cargo clippy`, `cargo fmt --check`, the tests, `cargo-audit`, `cargo-deny` on every push in the Rust workflow definition; `forge build`, `forge fmt --check`, and `forge test`, the TypeScript linter, `cargo-fuzz`, and end-to-end scenarios on clean runners each in a workflow definition of its own, authored once and complete by the ticket that first needs it, so no ticket amends a definition another ticket created | Workspace and discipline bootstrap for the Rust definition; `contracts/evm/PairingLib` for the Solidity definition; the first shell or webview ticket for the TypeScript definition; the first fuzz target and the first end-to-end scenario for theirs |
| Encoding family | `encoding/derivation_context`, the family-owned description of the first encodable domain type, authoring the vendor-free encoding contract and creating the crate; `encoding/abi`, implementing `IEncoderAdapter` and `IDecoderAdapter` through `alloy`'s sol types, which Solidity reproduces natively, and authoring them under one versioned encoding identifier with the declaration, proven over the derivation context and its identifiers, for everything hashed, signed, stored, or framed over IPC; `encoding/factory` following it | Pairing adapters and key derivation, after the domain's identifier modules and the derivation context |
| Randomness family | `random/os` over the operating system's generator, authoring the interface that fills bytes, the declaration, and the family's mock; `random/factory` following it with the family's integration test and the grouping's commit | Workspace and discipline bootstrap |
| Secret type | A `domain` type that cannot be formatted or serialized, exposes the wrapped value only through an explicit accessor, and zeroizes on drop through `zeroize`, a domain crate dependency | Workspace and discipline bootstrap |
| Telemetry family and tracing with correlation | `telemetry/local_metrics`, authoring the exporter interface; `telemetry/tracing`, per-request correlation identifiers across every subsystem; `telemetry/factory`; `telemetry/opentelemetry` as a further concrete, the factory revised in place | Daemon skeleton for the local metrics concrete, tracing, and the factory; observability reconciliation for OpenTelemetry |
| Storage engine family | `storage/redb`, authoring the key-value store interface with tables, transactions, and checkpoints; `storage/factory`; consumed by the job engine, the configuration registry, and every index | Swarm transport and seed host, at its first consumer |
| Local chain and test chain | Anvil in the developer loop; Base Sepolia for deployment; Foundry scripts under `contracts/evm` emitting addresses into configuration | Solidity verifier |
| Generated verifier constants and vectors | Solidity constants and test vectors produced from the Rust reference by `harness-crypto/generate/evm`, written under the configured generated directory when `harness-crypto/main` runs the generator, and committed, since the Solidity continuous-integration definition runs only Foundry | Delivery proof and Solidity verifier |
| Harness configuration | `harness-crypto/config`, the typed configuration the harness owns with a shipped default for every value it passes as params, the concrete per family, the curves to run, the target-group encoding identifier per curve, the forms identifier, the verifier form, the delivery-statement version, the network profile, the iteration counts, and the output paths, overridable from a configuration file and from command-line flags; `harness-crypto/main` reads it and composes every family through its factory | Delivery proof and Solidity verifier |
| Code signing | Apple, Authenticode, and Sigstore identities enrolled during the harness grouping so they exist before the onboarding grouping | Workspace and discipline bootstrap, enrollment started; installation coordinator, first use |
| Demonstration harness | Controlled participants, wallets, chain state, and fault injection; load-bearing under the completion boundary | Its own milestone in the shells and services grouping |
| Metrics store and release-evidence format | The RO-03 record and the harness output document | Daemon skeleton; harness report |

# Milestones

Within each grouping, milestones are listed in dependency order; where two are independent, the text says so.

## Foundation grouping

### Workspace and discipline bootstrap

**Purpose.** Turn the empty repository into a workspace where a node can be authored and proven on Windows, macOS, and Linux.

**Scope.** The tickets `workspace/cargo`, `domain/secret`, `random/os`, `random/factory`, and `workspace/ci`: the virtual workspace manifest with the glob members `crates/*`, `adapters/*`, and `apps/*`, the lint table, and the `[patch.crates-io]` overlay pointing `librqbit` at the project's overlay branch `chaintorrent-overlay` in `https://github.com/tsylvester/rqbit`, which exists at a tagged upstream release before the bootstrap as external setup; `rust-toolchain.toml`, `deny.toml` with the license allowlist and that repository as its allowed git source, and `.gitignore`; the `domain` crate, created by its secret module; the `adapters/random` crate, created by the operating-system concrete, which authors the byte-filling interface, the declaration, and the family's mock as its producers, with the factory following and carrying the family's integration test across factory and concrete; no encoding family here, since nothing encodable exists yet; no crate created ahead of the module that first lives in it, each configuration file authored once and complete by the ticket that creates it, and each crate's manifest and barrel receiving one entry per module from that module's ticket; the element mapping applied as one module directory per function, one adapter per file, and a family crate's `factory` module as its factory; the GitHub Actions matrix over Windows, macOS, and Linux running the `cargo` checks, the tests, and the audits; Apple, Microsoft, and Sigstore signing enrollment started. The manifests and the CI definition are configuration files with no types and no tests; `random/factory` carries the milestone's integration test and commit.

**Entry.** None.

**Exit.** The workspace and every crate its tickets create build and pass checks on Windows, macOS, and Linux; the lint table rejects the denied constructs in production code; `cargo-deny` enforces the allowlist; the secret type is proven at compile time to implement no formatting or serialization trait; each family crate exposes its factory's surface and no concrete; the operating-system concrete's repeated draws of a fixed width are pairwise distinct across a fixed count and a filled buffer of a fixed length holds more than one distinct byte value, the source alone being proven here, with CR-05's per-bootstrap independence proven at the First Finder engine and its reproducible publisher derivation at the explicit publisher path. Requirement rows: CR-05 for the randomness source, CR-07 for the secret type, XA-07 for the facilities, NF-M07 as the authoring discipline.

**Deliverables.** The workspace; the CI definition; the domain and random crates; the signing enrollment requests filed.

**Owner.** Project lead with the platform implementer.

**Unblocks.** Everything.

## Cryptographic validation harness grouping

Mapped at ticket resolution in the dependency map. The independent starting points are `pairing/bn254_arkworks`, which carries the pairing interface with its sampling bound, and the domain's identifier modules; the derivation context follows the identifiers, the encoding family follows the derivation context, and `kdf/blake3_keyed`, the first ticket that encodes, follows the encoding factory; every other ticket consumes one of them, and each family's factory follows every concrete of the family. The harness constructs every curve and verifier-form combination through the factories, so an invalid combination is refused here as the daemon's resolver will refuse it. External cryptographic review starts on the composition claims when this grouping starts.

### Pairing adapters and key derivation

**Purpose.** The group arithmetic and derivation every cryptographic ticket calls, on both curves, with encodings that match the precompiles, behind factories that carry their declarations from the first ticket.

**Scope.** `pairing/bn254_arkworks`, authoring `IPairingAdapter`, the group-element and scalar associated types with the sampling bound on the scalar type, sample from uniform bytes, the declaration of the curve, of second-group arithmetic at the verifier, of precompile encodings, and of the target-group encoding identifier, and the family's mock as its producers, owning its scalar and group-element types over its library's elements behind fallible constructors, implementing the sampling bound on its scalar type, proving the in-range draw, and creating the crate; `pairing/bn254_halo2curves`, `pairing/bls12_381_arkworks`, and `pairing/bls12_381_halo2curves`, one concrete per curve per library, each naming only its own library, each owning its scalar and group-element types over its library's elements, each implementing and proving the sampling bound on its scalar type, and each proven by executing the family's target-group encoding identifier's definition in its tests, the BLS12-381 concretes also against the CFRG draft's published value; `pairing/factory`, admitting a concrete only when it declares the target-group encoding identifier the suite requires and constructing the arkworks concrete as the default per curve, per the tech stack's assumption, unless the composition names one, revised in place if the benchmark selects otherwise; `harness-crypto/benchmark`, which constructs every pairing concrete through the factory, measures scalar multiplication, multi-scalar multiplication, and pairing on each, and records the default per curve, with every concrete remaining beneath the factory; the domain's identifier modules, `domain/asset_identity`, `domain/deployment_identity`, `domain/suite_identifier`, `domain/parameter_set_identifier`, `domain/group_index`, and `domain/piece_geometry`, each with its fallible `try_new` constructor enforcing its invariants, and `domain/derivation_context` composed of them, the first encodable domain type; `encoding/derivation_context`, the family-owned description of that type, authoring the vendor-free encoding contract as its producer and creating the encoding crate; `encoding/abi`, authoring `IEncoderAdapter` and `IDecoderAdapter` under one versioned encoding identifier, the declaration, and the family's mock as its producers and implementing both, its known-answer vectors the derivation context's and its identifiers' encodings authored from the ABI specification, its round trip and its rejections of truncated input, trailing bytes, non-canonical padding, and a wrong element count running over the same values, and adding to the encoding contract the byte-string kind that carries a group element's precompile encoding as ABI `bytes`; `encoding/factory`, carrying the family's integration test across factory, concrete, and description; `kdf/blake3_keyed` for every off-chain derivation over the encoded context, authoring the derivation interface and the KDF identifier, then `kdf/factory`; `hash-to-scalar/keccak256` for the identity mapping and the challenge, authoring the interface that maps domain-tagged bytes to a scalar and its identifier, then `hash-to-scalar/factory`.

**Entry.** Workspace and discipline bootstrap.

**Exit.** CR-10 vectors against precompile behavior including subgroup rejection and encoding edge cases on both curves, each curve constructed through the pairing factory, and each scalar type's draw from uniform bytes landing in its field; each pairing concrete encoding the pairing of the two generators to the identifier's definition executed in its tests, the two libraries of each curve encoding the same pairing to the same bytes, and the factory refusing a concrete whose target-group encoding identifier the suite does not require; the ABI concrete encoding the derivation context and its identifiers to their known-answer vectors, round-tripping them, and rejecting malformed input, and encoding a byte string to its known-answer vector; CR-11's derivation and hash-to-scalar produce the specification's outputs with context strings, serialization, and output lengths frozen, matching known-answer vectors from an independent implementation; the benchmark recorded over every pairing concrete and the default per curve recorded.

**Deliverables.** The pairing, encoding, KDF, and hash-to-scalar crates; the domain's identifier modules and derivation context; the benchmark record.

**Owner.** Cryptography implementer.

**Unblocks.** Credential KEM, envelope, delivery proof.

### Credential KEM

**Purpose.** The construction that makes decryption capability native and distinct per interval, and the wrap that lets every live set open one ciphertext.

**Scope.** `kem/bb1_depth_one`, implementing setup, issue, rerandomize, validity, encapsulate, well-formedness, and decapsulate as one adapter, owning its parameter-set, credential, capsule, and identity-element types, and authoring `ICredentialKemAdapter` with its associated types, the identity-scope declaration, and the family's mock as its producers; `kem/factory`, admitting a concrete by the suite's declared scope; `workflows/sidecar/wrap` and `workflows/sidecar/unwrap`, the wrapping key derived through the KDF family and the XOR wrap and unwrap of a piece-group key, which create the workflows crate.

**Entry.** Pairing adapters and key derivation, with every pairing concrete's definition test passing, since the encapsulated value's bytes are that test's subject.

**Exit.** CR-08 algebraic vectors: cross-holder agreement on the encapsulated value, cross-set agreement with two independently generated parameter sets unwrapping one piece-group key through the wrap, rerandomized-credential validity, cross-entitlement non-convertibility under the entitlement scope, holder-authored grants under the asset scope, malformed-capsule rejection, trivial identity-element refusal; CD-08's scope declaration refused by the factory where it must be.

**Deliverables.** The KEM crate; the workflows crate with the sidecar wrap and unwrap.

**Owner.** Cryptography implementer.

**Unblocks.** Envelope, whose wrap and unwrap carry the KEM's `CredentialComponents`; delivery proof; harness vectors.

### Envelope

**Purpose.** Delivery of a credential to a recipient's registered keys and nothing else's.

**Scope.** `envelope/possession_statement`, the family-owned description of a proof of possession's statement, the key's and the commitment's precompile encodings as byte strings, implementing the encoding contract and creating the envelope crate; `envelope/pairing_elgamal`, implementing key generation under proofs of possession, wrap, and unwrap as one adapter with identity-element and shared-secret rejection, each proof's challenge the hash-to-scalar mapping of the encoded possession statement, the key pair's secrets drawn by the caller and the coins and nonces through the randomness family, wrapping and unwrapping the KEM's `CredentialComponents`, owning its key-pair, public-key, possession, and envelope types, and authoring `IKeyAgreementAdapter` with those types as associated types with their fallible constructors, a recipient's public keys admitted only with their proof of possession, the envelope-algebra declaration, and the family's mock as its producers; `envelope/factory`. The validity check of an unwrapped credential belongs to the credential engine through the KEM factory, not to the envelope.

**Entry.** Pairing adapters and key derivation, including the encoding contract's byte-string kind; credential KEM, whose factory surface supplies `CredentialComponents`.

**Exit.** CR-04: independent coins, shared or identity secrets rejected, exact round trip, swapped keys or elements yield decryption failure or an invalid credential; LC-08's registration hygiene at the algebra level.

**Deliverables.** The envelope crate.

**Owner.** Cryptography implementer.

**Unblocks.** Delivery proof.

### Delivery proof and Solidity verifier

**Purpose.** The proof the contract verifies before it records an interval, in Rust as the reference and in Solidity as the shipped contract, bit for bit.

**Scope.** `encoding/abi`, revised in place with the fixed twenty-byte and 256-bit unsigned kinds the transcripts need, each named as a width against its own vector; `domain/asset_identity_hash`, the registry key the hash-card fixes, with its fallible constructor; `proof/mint_statement` and `proof/transfer_statement`, the family-owned descriptions of the mint and transfer transcripts under delivery-statement version one, carrying the purposes as declared data with their sixteen-bit codes and relations, generic over the chain family's identity, entitlement, interval, and chain-identifier forms with the entitlement contract in the identity form, the first authoring the encoding family's canonical-field contract in the encoding factory's interface and that family's form interface, each form bounded by that contract, and the forms' mock in `adapters/chain`'s factory module, and creating the `adapters/chain` and `adapters/proof` crates; `proof/schnorr_fs/challenge`, the Fiat–Shamir challenge the concrete owns, the hash-to-scalar mapping of a transcript's encoding under the proof's domain tag through the encoding and hash-to-scalar concretes the hash-card names; `proof/schnorr_fs`, the mint and transfer provers and the verifier in both forms as one adapter, declaring both forms and authoring `IDeliveryProofAdapter`, the algebraic statement types over the pairing's elements, the declaration of supported envelope algebras, verifier forms, and delivery-statement versions, and the family's mock as its producers; `proof/factory`, admitting a concrete only when it declares the resolved envelope's algebra, the verifier form the resolved pairing declares, and the delivery-statement version the hash-card names; `chain/evm_forms`, the EVM suite's identity, entitlement, interval, and chain-identifier forms over the kinds the suite's schemas fix as a private concrete of the chain family, authoring the forms portion of the family's declaration, a forms identifier with adapter and interface versions; `chain/create_chain_forms`, the family-owned function constructing the forms concrete the configuration names, admitting it only when its declaration carries the forms identifier the hash-card carries as an explicit field, supplied by the configuration in the harness and read from the deployment record in the daemon, and handing it to a consumer generic over the form interface, its own module because the factory holds one function and that function constructs the chain concrete; `harness-crypto/generate/evm`, generic over the form interface, rendering the Solidity constants and vectors from the Rust reference over the forms that function hands it with the statements' context values from the configured network profile, writing them under the destination its params carry, and authoring the generate family's interface, then `harness-crypto/generate/factory`; `harness-crypto/config`, the typed configuration the harness owns with a shipped default for every value it passes as params, overridable from a configuration file and from command-line flags; `harness-crypto/main`, the binary entry that reads the configuration and composes every family through its factory, running the benchmark and the generator and revised in place as later harness tickets add their runs; `contracts/evm/PairingLib` over EIP-196, EIP-197, and EIP-2537 with contract-side subgroup checks, its constants and vectors generated, creating the Foundry configuration and the Solidity continuous-integration definition with `forge build`, `forge fmt --check`, and `forge test`; `contracts/evm/DeliveryVerifier`; `contracts/evm/deploy` to Anvil and Base Sepolia; `harness-crypto/verifier/evm`, which reaches the deployed verifier and reads acceptance and cost and authors the verifier family's interface, then `harness-crypto/verifier/factory`.

**Entry.** Credential KEM and envelope.

**Exit.** CR-09 in both verifier forms with every statement field and response mutated and replay across settlements rejected; CD-03 on each curve form with record-contradicting statements rejected; the Rust verifier and the deployed contract agree on every vector; the hash-to-scalar identity-mapping and challenge vectors mirrored to Solidity by the generator and reproduced by the deployed verifier; the proof factory refuses each curve and verifier-form combination it must and a hash-card naming a delivery-statement version no description supports; the family-owned forms function refuses a forms concrete whose identifier the suite does not require and hands the admitted one to a consumer that never names it; the harness binary composes every family through its factory from the configuration's values, with a file and a flag override each shown to replace a default; the generated directory written by that run before the pairing library's tests read it and carried by the milestone's commit; the verifier deployed on Base Sepolia with its address recorded in the configuration's deployment record.

**Deliverables.** The proof crate with its transcript descriptions, the chain crate with its form interface, its EVM forms concrete, and its family-owned forms function, the asset-identity-hash module, the revised abi concrete, the harness configuration and binary, the committed generated directory, the Solidity pairing library and verifier, the generate and verifier families with their EVM concretes, the Base Sepolia deployment record.

**Owner.** Cryptography implementer with the contract implementer.

**Unblocks.** Harness measurement; registry and entitlement contracts.

### Harness vectors, measurement, and report

**Purpose.** Measure what nothing has measured, fix the parameters that gate every node that encrypts, and calibrate throughput.

**Scope.** `harness-crypto/vectors`, constructed through the factories, generic over the form interface, and including the admission vectors the proof and chain factories must refuse, `harness-crypto/measure`, `harness-crypto/report`, each writing its output under the paths the configuration names; `harness-crypto/main` revised in place to run them from the configuration's values; the report node carries the grouping's integration test across the whole chain and its commit.

**Entry.** Delivery proof and Solidity verifier.

**Exit.** CD-07 and AS-21: capsule, envelope, and proof sizes; decapsulation time per piece group; proof generation and verification time; delivery cost as L2 execution gas and L1 data fee for a mint and a transfer, on both curves, the L1 fee noted as Sepolia's; the measurements, the piece-group size and curve defaults in force, and any adjustment the data supports recorded in release evidence; tickets closed per unit of time recorded across the grouping.

**Deliverables.** The release-evidence document; the throughput record; the first evidence-based estimate of the remaining work, produced by re-mapping the remaining protocol core milestones to tickets.

**Owner.** Cryptography implementer; project lead for the default adjustment, the re-map, and the funding decision.

**Unblocks.** The re-map and the funding stop criterion.

## Protocol core and contract suite grouping

The hashing, signature, and swarm milestones are at ticket resolution and run beside the harness from the encoding family's closure onward, since their first tickets encode and they depend on nothing the harness measures; the remaining milestones are re-mapped to tickets when the harness report closes.

### Hashing and commitments

**Purpose.** The integrity layer the specification separates from confidentiality deliberately.

**Scope.** `hashing/blake3_bao`, implementing root and outboard construction, streaming and random-access verification, and random challenge and response as one adapter with keyed mode for the keyed plaintext-root disclosure mode, and authoring the commitment interface, the root, outboard, path, and chunk types, the commitment-scheme identifier the hash-card carries, the commitment-scheme and chunk-granularity declaration, and the family's mock as its producers; `hashing/factory`, which carries the milestone's integration test and commit.

**Entry.** The encoding family's closure at `encoding/factory`, inside pairing adapters and key derivation. Independent of the rest of the harness.

**Exit.** CR-02 corruption of roots, paths, chunks, lengths, and order detected across construction, verification, and challenge, through the factory.

**Deliverables.** The hashing crate.

**Owner.** Cryptography implementer.

**Unblocks.** Swarm transport and seed host; payload cipher and sidecar layer; the plaintext CAS.

### Payload cipher and sidecar layer

**Purpose.** The symmetric layer, and the sidecar that lets every live parameter set open one ciphertext.

**Scope.** `cipher/aes_ctr` with the 64/64 counter layout, continuous-stream addressing, extent and index bounds, authoring `IPayloadCipherAdapter`, the counter-layout and addressable-extent declarations, and the family's mock as its producers; `cipher/factory`; `workflows/sidecar/build`, per live parameter set a capsule and wrapped piece-group key per group committed by that set's Bao root, each sidecar its own object; `workflows/sidecar/validate`, manifest, hash-card, and sidecar validation in the ordering the specification requires; the decapsulation-to-cipher seam; the `sample_deployment` generator for the site demonstration, which needs the cipher and so lives here rather than in the harness.

**Entry.** Hashing and commitments; the credential KEM milestone, for the wrap. The `sample_deployment` generator carries the configured piece-group size default.

**Exit.** CR-01 vectors including maximum-valid counters and overflow rejection; CR-06 fuzzing with chain and custody spies proving validation precedes any credential exercise; a two-set sidecar opening one ciphertext under a credential of either set; the sample deployment bundled for the WebAssembly build.

**Deliverables.** The cipher crate; the validation workflow; the sample deployment.

**Owner.** Cryptography implementer.

**Unblocks.** Decryption pipeline, First Finder engine, the site demonstration.

### Signature services

**Purpose.** Per-layer signatures behind one factory that resolves a scheme per layer.

**Scope.** `signature/ed25519` with domain separation and complete-message binding, authoring `ISignatureAdapter`, the signature types, the per-layer declaration of scheme and layers served, and the family's mock as its producers; `signature/secp256k1` through `alloy` with EIP-712 typed data for registrations, bindings, and signed intents; `signature/factory`, resolving a scheme per layer, which carries the milestone's integration test and commit. The binding schema with the DID Document types belongs to the identity family and is authored in the identity and custody milestone.

**Entry.** The encoding family's closure at `encoding/factory`, inside pairing adapters and key derivation. Independent of the rest of the harness and of the cipher milestone.

**Exit.** CR-03: valid signatures verified; cross-domain, cross-layer, truncated-context, wrong-scheme, and replay substitutions rejected, each scheme resolved through the factory for its layer.

**Deliverables.** The signature crate.

**Owner.** Cryptography implementer.

**Unblocks.** Identity and custody; contract binding surface.

### Registry and entitlement contracts

**Purpose.** The ledger the protocol's decentralized properties rest on.

**Scope.** The EVM suite under `contracts/evm`, the on-chain concrete of the chain family's Base concrete, from the technical requirements' API surface: registry with asset records and the per-name version index, deployments carrying every hash-card field, the sidecar-per-live-set rule, sidecar addition for later sets, attestation verification through the `IAttestationVerifier` adapter the adapter registry binds per ingest source, with the npm P-256 verifier over the source-key table as the first concrete and a recorded absence admitted, and later escrow deployments of an existing asset; parameter-set liveness and retirement; envelope-key registry with proofs of possession; `IEntitlement` with its ERC-721 concrete carrying interval state and envelope digests and the ownership override that disables the standard transfer and approval entry points; batched grant requests readable by holders and closed by grant or withdrawal; locks with expiry and refund; mint, deliver, grant calling the harness's verifier; the identity contract account under ERC-1271 with its device registry, admission, re-scoping, revocation, and the `isDeviceAdmitted` view; every identity-bound mutation taking the acting identity and a signed intent verified through the account against a signer whose permissions cover it, with lock expiry enforced and no volume limit; `evaluateAuthorization` and its paginated batch; escrow records per deployment with provenance, attestation or its absence, and no maintainer commitment; identity binding; claim settlement state layout and verifier-key registry; `IIdentityAdapter` with the publisher-authority and escrow-identity adapters; the adapter registry and factory with constructor-injected, immutable bindings under the project-held governance key, binding the identity adapters, the signature adapters, the attestation verifier per source, and the entitlement token form. Deployment scripts to Anvil and Base Sepolia.

**Entry.** Delivery proof and Solidity verifier.

**Exit.** Every transition of LC-01 exercised through Foundry tests with asserted events and views, the Rust form of that proof belonging to the chain and submission families milestone; LC-02, LC-05, LC-07, LC-08, LC-09 including sidecar addition, LC-10, LC-11 with owner, approved address, and operator each refused an ordinary transfer, LC-12 with batched requests opened, fulfilled, withdrawn, and shown to authorize nothing, LC-13 through the identity contract account as paymaster-sponsored, relayer-submitted, and self-funded calls, with foreign, expired, replayed, revoked-signer, and out-of-permission intents refused; AS-13's proof-set acceptance and rejection on Base Sepolia.

**Deliverables.** The contract suite deployed on Base Sepolia; addresses in configuration; Foundry fuzz and invariant tests; static analysis in CI.

**Owner.** Contract implementer.

**Unblocks.** Chain adapters; First Finder engine; credential delivery; claim.

### Chain and submission families

**Purpose.** The daemon's only view of consensus, from a quorum of configured nodes, and the one path a signed intent takes to the contracts.

**Scope.** `chain/base`, the contract bindings through `alloy`, Base's tier mapping to INCLUDED, SOFT, HARD, and SETTLED with reference age, the entitlement-state views, and the events reader for envelope recovery from calldata and events, authoring the chain interface, `ISettlementAdapter`, `IEntitlementStateAdapter`, the remainder of the declaration, and their mock as its producers in the factory module the proof family's mint transcript created, and binding as its associated forms the forms `create_chain_forms` resolves, its declaration naming the forms identifier it binds; `chain/quorum_view`, the family-owned aggregation of single and paginated views as a quorum, two of three configured nodes agreeing at a common reference within `τ_soft`, stale nodes ignored, divergence and over-age views failing closed, shared by every chain concrete; `chain/factory`, the chain factory function constructing the chain concrete the configuration names, admitting it only when its declaration lists the forms identifier the resolved forms carry, and its `base` branch, authored once after the concrete; `submission/self_funded`, the concrete the daemon uses on Base Sepolia until the relayer exists, authoring the interface for submitting a signed intent and its declaration; `submission/factory`, revised in place when the relayer milestone adds its concretes; `discovery/seeder_map`, the on-chain seeder map as a discovery source through the chain factory, with `discovery/factory` revised in place to carry its branch; intent signing through the signature factory.

**Entry.** Registry and entitlement contracts; signature services.

**Exit.** LC-01 every transition exercised through the chain factory's resolved Base concrete with asserted events and views; LC-04 included, pending, reorganized, stale, and insufficient-tier references driving the attempt rule for a zero-price and a priced deployment; LC-06 batch parity with individual evaluation; XA-06's decision table, one timeout, one stale provider, honest heads at different heights, divergent state at one reference, a reorganization, and a prolonged stall, each with its single defined result; CD-04 recovery from chain history; LC-13's self-funded call path through the submission factory.

**Deliverables.** The chain crate completed with its Base concrete, its quorum view, and its factory; the submission crate; the discovery family's seeder-map concrete.

**Owner.** Chain adapter implementer.

**Unblocks.** Credential delivery; identity; relayer; every state read.

### Swarm transport and seed host

**Purpose.** Bytes moving between machines with nothing readable in transit, provable with no chain and no KEM.

**Scope.** `transport/rqbit`, over the overlay branch the workspace manifest names, with the concrete's hooks and the upstream issues opened, root-to-infohash translation, torrent creation per object with the ciphertext and each sidecar as its own object and locator, Bao verification of every completed piece after the library's SHA-1 check through the hashing factory, seeder-map peer injection, and the engine capability, authoring `ISwarmTransportAdapter`, the locator types, the declaration, the engine capability a transport may expose, and the family's mock as its producers; `transport/factory`; `discovery/local`, authoring `IPeerDiscoveryAdapter`, the peer types, the declaration, and the family's mock; `discovery/dht`, `discovery/pex`, and `discovery/tracker` through the transport's engine capability; the family-owned `discovery/aggregate`; `discovery/factory`; `storage/redb`, at its first consumer, authoring the store interface; `storage/factory`; `seed-host/owned` with the root-keyed ciphertext store, holding reason, quota, eviction, and its index through the storage factory, settings passthrough for the controls the library has and enforcement of the ones it does not, authoring `ISeedHostAdapter`, the capacity and holding-reason declaration, and the family's mock; `seed-host/rqbit`, driving the rqbit application through its HTTP API with root-to-identifier translation and Bao custody challenges through the hashing factory; `seed-host/factory`, which carries the milestone's integration test and commit.

**Entry.** Hashing and commitments, and through it the encoding family's closure. Independent of the rest of the harness and of every contract milestone; `discovery/seeder_map` is authored in the chain and submission families milestone, which revises the discovery factory in place.

**Exit.** SW-01 through SW-07 and EC-02 as amended: retrieval through the owned host and a delegated external client with identical roots; a corrupted piece passing SHA-1 rejected by Bao before storage; discovery aggregation deterministic under conflicting and malicious results; seeding continuity across host restart while another participant retrieves; delegated custody challenges detecting false reports; resumable failure handling; the archive visible.

**Deliverables.** The transport, discovery, storage, and seed-host crates; the overlay branch; the upstream issues.

**Owner.** Daemon implementer.

**Unblocks.** First Finder engine; package serving from the swarm; the project seed host.

## Local daemon and package serving grouping

Re-mapped to tickets when the protocol core grouping closes. The identity milestone runs early because the installation coordinator needs it.

### Daemon skeleton: jobs, configuration, settings, resolver, health, telemetry, IPC

**Purpose.** The process that owns the stores and the two ingress surfaces, with every cross-cutting mechanism present before any workflow lands.

**Scope.** `telemetry/local_metrics` over the storage factory with the RO-03 record, authoring the exporter interface, the declaration, and the family's mock as its producers and creating the `adapters/telemetry` crate; `telemetry/tracing`, the family-owned tracing setup carrying per-request correlation identifiers; `telemetry/factory`, revised in place when observability adds its OpenTelemetry concrete; `workflows/jobs` over the storage factory with checkpoints, idempotency keys, restart, and cancellation; `workflows/config` over the storage and encoding factories, and `workflows/settings` with the catalogue including its presence-required marks and the hedge delay, source deadline, and grant wait, validation, migration jobs, backup and restore, export and import as a versioned TOML document; `platform-paths/linux`, authoring the application-directory interface, `platform-paths/macos`, `platform-paths/windows`, and `platform-paths/factory` for the store-root defaults; `workflows/compose`, the composition resolver that works back through every factory's declarations; `workflows/health`; the family-owned `ipc/framing` and `ipc/principals`, `ipc/unix_socket` authoring the IPC transport interface, `ipc/named_pipe`, `ipc/server` with the control-principal rule and the local-presence confirmation, and `ipc/factory`; `lifecycle/lock`, the single-instance lock as a family-owned function creating the `adapters/lifecycle` crate, the family's concretes and factory following in the installation coordinator; `apps/daemon` binary skeleton.

**Entry.** Workspace and discipline bootstrap; signature services for the settings that reference custody; the storage family from the swarm milestone. Independent of the contract milestones.

**Exit.** XA-03 termination at every persisted transition with exactly-once effects; IC-04, IC-05 with no side effects from invalid compositions, IC-08; ST-01 through ST-04 and ST-08 through the IPC; XA-02 other local users gain nothing, a lifecycle script running as the installing user obtains resolution only, and every local-presence operation and presence-required setting waits on the daemon-owned interactive confirmation; XA-10 backoff within declared ceilings; RO-04 correlation across the skeleton.

**Deliverables.** A daemon that starts, resolves a composition, holds settings, reports health, and refuses what it must.

**Owner.** Daemon implementer.

**Unblocks.** Every daemon workflow; the installation coordinator.

### Identity and custody

**Purpose.** The principal, its keys, and where they live.

**Scope.** The family-owned `custody/holder_seed`, deriving the control branch, holding the handshake key, and the envelope branch, holding the envelope secrets, on a root only through the KDF family, creating the `adapters/custody` crate; `custody/device_key`, generation on every device through the signature and random families; `custody/local_keystore` over `keyring` with a wrapping key and version-headed encrypted blobs, the only module touching the OS credential store, authoring `IKeyCustodyAdapter`, the versioned capability set that declares root and reader and also names read-delegate and signer, and the family's mock as its producers; `custody/upgrade`, the in-place migration job between custody concretes through the interface, verified against the on-chain binding and envelope-key registration; `custody/factory`; `wallet/eip1193`, authoring the signing-only wallet interface, `wallet/walletconnect`, and `wallet/factory`; `identity/publisher_authority`, authoring `IIdentityAdapter`, the binding schema with the DID Document types and anchor hash, and the proof-class declaration, `identity/escrow`, and `identity/factory`; `workflows/identity` create or import, relayer-paid binding and identity contract account creation through the chain and submission factories, envelope-key registration with proofs of possession, device pairing by QR code or short authentication string, signer admission, re-scoping, and revocation on the contract account, role delegations under the handshake key, envelope secrets delivered to a reader over the paired channel, signing-only wallet connections through the wallet factory; the persistent credential store per entitlement; the secret region types.

**Entry.** Daemon skeleton; signature services; the chain and submission families, for binding and registration on Base Sepolia.

**Exit.** IW-01 through IW-07: distinct schemes composed and surviving restart; capability combinations failing closed; one identity, one binding, one registration across restart and reinstall; no decrypted credential or key in any custody surface; a reader device paired to a root on a second device profile reading alone with the seed never leaving the root, then revoked and purging its envelopes and envelope secrets at its next state view; recovery of the root on a fresh profile from the seed; an in-place custody upgrade preserving the principal, and a tampered rewrap rolled back; wallets signing only; envelope keys never wallet keys.

**Deliverables.** The custody, wallet, and identity crates; the identity workflow.

**Owner.** Daemon implementer with the cryptography implementer for derivation.

**Unblocks.** Entitlement acquisition; the installation coordinator; the publisher engine.

### Plaintext CAS, resolution orchestrator, and package host

**Purpose.** The benefit a developer feels, served before any cryptography is exercised on the read path.

**Scope.** `cas/filesystem` with the content-hash layout of verified tarballs under the repointable root, atomic commit, eviction warnings, its index through the storage factory, and nothing linking into the store, authoring the store interface with commit, serve, pin, quota, eviction, and reuse metrics as its producers; `cas/factory`; `workflows/resolve` trying sources in order and hedged under the catalogue's hedge delay, source deadline, and grant wait, upstream for any identity without a credential, and path and reason metrics; `workflows/health` independence status per asset and per project; `package-host/npm` serving packuments from its metadata store and tarballs on the loopback port, with canonical tarball URLs for lockfile portability and the metadata capture at every upstream resolution, authoring `IPackageHostAdapter` and the ecosystem declaration as its producers, then `package-host/factory`; the upstream plaintext-root check against the canonical record through the hashing factory; store migration jobs for ST-02.

**Entry.** Daemon skeleton.

**Exit.** PR-01 parity with upstream across representative projects; PR-02 deterministic mapping; PR-03 each source chosen and its reason recorded with and without a held credential and with upstream available and unavailable, the hedge start, the bounded wait, and the pending failure asserted; PR-04 under interruption, races, and corruption; PR-05 lifecycles separate; PR-06 and AS-06 plaintext hit with everything else unreachable; PR-08 the foreground install's latency unchanged with every background stage stalled; PR-09 a lockfile committed on one machine installing on another with a different host port, and the metadata store serving with a staleness marker when upstream is unavailable; PR-12 independence shown identically on every surface; ST-02 migration with interruption and prior projects still installing afterward.

**Deliverables.** The CAS and package-host crates with their npm and filesystem concretes; the resolution workflow.

**Owner.** Daemon implementer.

**Unblocks.** First Finder engine.

### First Finder engine

**Purpose.** Absent packages enter the swarm without slowing the install that fetched them.

**Scope.** `ingest/npm` with as-is fetch, registry-signature attestation validation against the registry's published keys or recorded absence, metadata capture, and public-availability eligibility, authoring `IIngestSourceAdapter` and the attestation-presence declaration as its producers; `ingest/factory`; `workflows/first_finder` foreground serve and background durable job: deployment identity under state lock, master scalar and parameter set through the KEM factory, a piece-group key and capsule randomness per group and the IV through the random factory, encryption per group through the cipher factory, one sidecar per live set through `workflows/sidecar/build` and the hash-card, registration race with loser destruction, escrow custody of the master scalar, seed handoff through the seed-host factory, and the finder's grant of the asset's first entitlement to itself; `workflows/grant`, the grant service under the asset scope.

**Entry.** Plaintext CAS and package host; swarm transport and seed host; registry contracts; the chain and submission families; identity and custody; payload cipher and sidecar layer; the credential KEM, envelope, and delivery proof families for the self-grant.

**Exit.** FF-01 through FF-07 and AS-09, AS-10, AS-12: verified bytes served with every background dependency unavailable, forged and mismatched attestations refused and a missing one recorded; single completion after termination at every checkpoint; production randomness validated; race with one winner and clean losers, and a later escrow deployment of an existing asset admitted; the master scalar retained under custody; the self-grant recorded once. The grant service under the asset scope is built here and proven at the credential delivery milestone, whose producers it needs.

**Deliverables.** The ingest crate and First Finder workflow.

**Owner.** Daemon implementer.

**Unblocks.** Credential delivery; explicit publishing.

### Credential delivery and per-attempt authorization

**Purpose.** Close the read path: nothing requested from anyone at read time, and nothing decrypted without a current view.

**Scope.** `workflows/request`, one batched sponsored request per install for every unheld ledger-known asset, queued until submittable, with pickup of grants from chain events on any later run; `workflows/prefetch`, ciphertext and sidecar while a request is pending, under the seeding settings and quota, pinned, seeded, and matched to the granted set; `workflows/acquire` with relayer-paid mint, escrow grant, and funded purchase with lock; the grant service fulfilling open requests for held escrow assets as it seeds, requesters present or absent; the credential engine decrypting envelopes into the secret region through the envelope factory, the validity check through the KEM factory, in-memory rerandomization, zeroization; `workflows/attempt` with `AttemptContext`, freshness clocks, the wallet-control assertion by a device key the identity's contract account admits in the same state view, three-state handling, batch form; `workflows/decrypt` with decapsulation through the KEM factory, unwrap through `workflows/sidecar/unwrap`, AES-CTR addressing through the cipher factory, Bao verification through the hashing factory, CAS commit, streaming; the interval-end watcher; the deployment gate; sale delivery from a fresh decryption with the transfer proof through the proof factory.

**Entry.** Identity and custody; the chain and submission families; the KEM, envelope, and proof families; payload cipher and sidecar layer; registry contracts; First Finder engine; `submission/relayer` once the relayer exists, `submission/self_funded` on Base Sepolia until then.

**Exit.** EC-01 through EC-08 and AS-07, AS-08 with the ingest source disabled, AS-19; CD-01, CD-02, CD-04, CD-08 and AS-14; CD-05's automatic grant service with the First Finder offline, a requester offline when its grant is authored and loading it on its next run, and a swarm of only non-holder seeders reporting no grant author truthfully, AS-26; PR-10 and PR-11 through AS-29, the first run ending independent with every branch exercised, held, absent, dead-deployment, explicit-publisher, and priced assets; the delivery scenario AS-13 through the daemon rather than the harness; the grant-request and grant-pickup integration points crossed; storage and crash artifacts free of decrypt-capable material; destruction at HARD and a reorganized transfer leaving the seller able to read; a reader device denied and purging at its next state view after its signer is revoked.

**Deliverables.** The acquisition, credential, attempt, decryption, and interval workflows.

**Owner.** Daemon implementer with the cryptography implementer.

**Unblocks.** The demonstrable milestone; the transaction flow proof.

### Demonstrable milestone: an install resolves against the swarm

**Purpose.** The earliest point the north star and install wall-clock can be observed on the dogfood population, and the first thing the project can show.

**Scope.** No new subsystem. A second clean identity on a second machine profile installs a pinned closure the first ingested, at the registry's speed with the requests registered and the ciphertext prefetched, and then, with the registry unavailable, through the packaged daemon and package host over the swarm, on a grant from the first identity fulfilled while the second was offline.

**Entry.** Credential delivery and per-attempt authorization; plaintext CAS and package host; First Finder engine; swarm transport and seed host.

**Exit.** FF-08 and AS-11, AS-24, and AS-29 in substance; the north star, cross-project reuse, and the fraction of the closure independent after the first run measured on the dogfood population; whole-command elapsed time and authorization fraction reported against baseline, with any adjustment of the configured defaults the data supports.

**Deliverables.** The dogfood measurement record.

**Owner.** Project lead.

**Unblocks.** Confidence; nothing technical.

## Onboarding shells and services grouping

Re-mapped to tickets when the daemon grouping closes. The relayer and the publisher path are independent of the installer milestones and may run in parallel with them, and the relayer's entry is the chain and submission families alone, so it may start before the daemon grouping closes; the grouping gate in the master plan says the same.

### Installation coordinator and platform adapters

**Purpose.** Either entry point produces one identical working system on a clean machine after only permitted consent.

**Scope.** `workflows/install` durable checkpointed plan; `lifecycle/systemd`, authoring the service-lifecycle interface, `lifecycle/launchd`, `lifecycle/windows_service`, `lifecycle/scheduled_task`, and `lifecycle/factory`, in the crate the daemon skeleton's lock created; `artifact-verifier/sigstore`, authoring the signed-artifact verification interface, `artifact-verifier/authenticode`, `artifact-verifier/apple_notarization`, and `artifact-verifier/factory`, for artifact download and verification before execution; `redirect/npm` across user, project, workspace, proxy, and custom-registry cases, authoring the detect, redirect, back-up, and restore interface, then `redirect/factory`; backup and rollback; idempotent reinstall; repair; signed reversible update; uninstall with preservation; initial settings from the catalogue with path and consent items offered, including the request and prefetch items with the first-run cost disclosure and the cache-only mode; the reserved account-link step shown as unavailable; the first-run disclosure; `apps/installer`.

**Entry.** Daemon skeleton; identity and custody; the signing identities enrolled at bootstrap.

**Exit.** SI-03 through SI-18 and SI-20 and IC-01 through IC-03, IC-06, IC-07; ST-07; AS-03, AS-04, AS-05, AS-18, AS-30 on every supported platform, each platform's lifecycle and verification resolved through its factory; the consent trace clean under SI-19.

**Deliverables.** The installer binary; the lifecycle concretes and the artifact-verifier and redirect crates; per-platform packages through `cargo-dist`.

**Owner.** Platform implementer.

**Unblocks.** The shells; acceptance.

### Relayer or paymaster

**Purpose.** A first-run user never acquires gas to install a free package.

**Scope.** `apps/relayer` with the chosen ERC-4337 bundler and paymaster provider's concrete, authoring the bundler-provider interface, and `relayer/bundler/factory`, the provider evaluated here; the sponsorship mechanism for the identity contract account chosen here under LC-13's signed-intent rule, with `submission/paymaster` and `submission/relayer` authored as the chosen mechanism requires, `submission/factory` revised in place to carry their branches, and sponsorship overhead measured; sponsor endpoints for the one binding, device admission and revocation, one batched request per install, free mints, and fulfilling grants; admission policy with a global per-window budget and maximum sponsored liability first and per-identity rate limits sized to a realistic closure as one layer; explicit failure states naming exhaustion and denial; cost and budget reporting with L2 execution and L1 data fee separated; the grant pool sized from harness and dogfood gas.

**Entry.** The chain and submission families; registry contracts. Independent of the daemon grouping's closure.

**Exit.** RO-01 clean free onboarding, a device admission and a revocation sponsored, a batched request sponsored and its fulfilling grants sponsored, subsidy exhausted and denied with actionable recovery, requests queued locally, and no false authorization; LC-10 and AS-27 a flood of fresh identities never spending beyond the configured budget and exhaustion reported truthfully; RO-02 paid acquisition refused without funds.

**Deliverables.** The relayer service with its bundler factory and the chosen provider's concrete, the sponsored submission concretes, and its operating runbook.

**Owner.** Relayer operator.

**Unblocks.** Free onboarding end to end; the transaction flow proof.

### CLI and desktop application

**Purpose.** The two control surfaces the project owns outright, over the daemon's IPC.

**Scope.** `apps/cli` with `clap`: consent, settings, diagnostics, package-host control, identity, jobs, repair, uninstall; `apps/desktop` on Tauri 2: configuration, identity and custody workflows including device pairing, admission, and revocation, the custody upgrade, and root recovery, adapter selection, jobs, the seeding archive, the settings catalogue, diagnostics, repair, recovery.

**Entry.** Installation coordinator; daemon skeleton.

**Exit.** IC-08 and RO-05 identical facts across surfaces; ST-05 surface parity for every catalogue entry; SW-07 the archive view matching the host; SI-16 repair from each surface; IC-10 accessibility baselines.

**Deliverables.** The CLI and desktop packages.

**Owner.** Platform implementer.

**Unblocks.** Acceptance scenarios that exercise every surface.

### Visual Studio Code extension and npm bootstrap package

**Purpose.** The two onboarding paths the specification names, as thin shells.

**Scope.** `shells/vscode-extension` in TypeScript over the IPC client, bundling or fetching the signed installer, status and consent views, no protocol logic; `shells/npm-bootstrap` as a `postinstall`-free package whose `bin` invokes the signed installer.

**Entry.** Installation coordinator; CLI and desktop, for the IPC client patterns.

**Exit.** SI-01 and AS-01; SI-02 and AS-02; both paths converging on one postcondition, IC-01.

**Deliverables.** The extension and the npm package, published to their hosts' test channels.

**Owner.** Platform implementer.

**Unblocks.** Acceptance.

### Explicit publisher path and issuance policy

**Purpose.** The project publishes its own package, seeding the swarm with its first content and supplying the only assets that may carry a price.

**Scope.** `workflows/publish` with seed-derived parameter sets and capsule randomness through the KDF factory, `publisher-proof/provenance`, authoring `IPublisherProofAdapter` and the proof-class and strength declaration, `publisher-proof/maintainer_oauth`, and `publisher-proof/factory` resolving them in strength order, proof class recorded on chain, dependency-closure ingestion as First Finder, the issuance policy service serving mints automatically under configuration with per-request approval as a declared capability; the publisher seed under custody.

**Entry.** First Finder engine; identity and custody; registry contracts.

**Exit.** PC-01, PC-03 replay and redirection rejected, FF-09 and AS-22, AS-23; a second identity consumes the published package; the issuance policy serves a mint with no person present.

**Deliverables.** The publish workflow and the publisher-proof crate; ChainTorrent's own package published on Base Sepolia.

**Owner.** Daemon implementer.

**Unblocks.** The transaction flow proof; the project seed host's content; the claim verifier's inputs.

### Claim verifier and escrow claim

**Purpose.** A maintainer takes complete ownership of escrowed records without the First Finder and without any stored maintainer binding.

**Scope.** `apps/claim-verifier` running provenance-then-OAuth verification through the publisher-proof factory, establishing the claim set from upstream metadata at verification time, signing vouchers bound to claimant, set, contract, chain, nonce, and expiry under an on-chain registered key with rotation and revocation; `claim-verifier/attestor` as the daemon's client of the attestor service, authoring `IClaimVerifierAdapter` and the voucher types, then `claim-verifier/factory`; the escrow claim contract surface in the registry; `workflows/claim` in the daemon; claimant parameter-set registration, optional encrypted handover with compatibility proof, successor deployments with a sidecar per live set, sidecar addition to live escrow deployments from an escrow-era credential, voluntary migration and retirement.

**Entry.** Explicit publisher path; registry contracts; credential delivery.

**Exit.** PC-04 through PC-07 and CD-06; AS-16 claim with the First Finder absent, successor readable under both sets, the escrow deployment opening under the claimant's set after sidecar addition, one holder migrated; AS-27's revoked-key voucher rejected.

**Deliverables.** The verifier service; the claim-verifier crate; the claim workflow; the contract surface deployed.

**Owner.** Claim verifier operator with the contract implementer.

**Unblocks.** Acceptance.

### Project seed host, site, and WebAssembly demonstration

**Purpose.** A seed of the core closure from day one and an education tier a prospective developer can try with nothing to sign up for.

**Scope.** The daemon's seed host configured for persistence, seeded by the dogfood publication, and holding an entitlement and credential for every asset it seeds; the static site; `apps/wasm-demo` built with `wasm-bindgen` from the domain, hashing, pairing, KEM, and cipher crates, running against the sample deployment generated at the payload cipher milestone; the download links; the statement that seeding and the CAS do not run in a browser.

**Entry.** Explicit publisher path; swarm transport and seed host, for the owned concrete; payload cipher and sidecar layer, for the sample deployment.

**Exit.** The core closure retrievable from the seed host after publication, FF-09; SW-08, a new identity obtaining a grant for a core-closure asset from the seed host with every other holder offline; the demonstration performing real resolution, verification, and decapsulation with no network access on the read path; the protocol resolving with the seed host unreachable.

**Deliverables.** The seed host deployment; the site; the demonstration bundle.

**Owner.** Project lead with the daemon implementer.

**Unblocks.** Cold start; the prospective-developer audience.

### Observability reconciliation

**Purpose.** Every metric the economics depend on, reconciled against induced activity, with no secret anywhere.

**Scope.** Completion of the RO-03 record across every emitting subsystem; RO-04 correlation across concurrent installs; RO-06 attempt latency and install wall-clock measured and reported against the configured values in force, proven by injected delay reflected accurately; `telemetry/opentelemetry` as a further exporter concrete, with `telemetry/factory` revised in place to carry its branch; the telemetry scan; secret-free failure messages under XA-04.

**Entry.** Every daemon and service milestone, since each emits.

**Exit.** RO-03 through RO-06 and XA-04; AS-20 in rehearsal.

**Deliverables.** The reconciled metrics schema and scan.

**Owner.** Observability implementer.

**Unblocks.** The acceptance run.

### Demonstration harness

**Purpose.** Reproducible participants, wallets, chain state, and failures, without which the completion boundary cannot be met.

**Scope.** `apps/harness-demo`: spawned daemons under distinct identities and machine profiles, Anvil and Base Sepolia chain state, wallets funded and unfunded, fault injection at process, network, disk, and chain, the race and reorganization drivers, the deliberately incompatible adapter declarations the incompatibility scenario rejects, refused by the factories and by `workflows/compose`, the clean-machine runner integration for CI.

**Entry.** Daemon skeleton; runs alongside every milestone from the daemon grouping onward and closes here.

**Exit.** XA-07 every requirement's proof class mapped to a facility; XA-03 termination at every persisted transition driven by the harness; every scenario from AS-06 onward runnable unattended.

**Deliverables.** The harness and its scenario definitions.

**Owner.** Test infrastructure implementer.

**Unblocks.** The acceptance run.

## Acceptance and release grouping

### Transaction flow proof on the Base mainnet pilot

**Purpose.** Prove a nonzero transaction settles, on the one asset class where pricing is legitimate, while the entitlement graph is two identities.

**Scope.** Contract suite deployed to Base mainnet; ChainTorrent's package published and priced trivially above gas under the issuance policy; a funded buyer acquires; the holder resells; the settlement boundary asserted; every guardrail live.

**Entry.** Explicit publisher path; credential delivery; relayer; registry contracts on mainnet; external cryptographic review dispositioned, which gates any priced deployment; legal review of priced entitlements and the paymaster.

**Exit.** LC-03 First Finder pricing refused; LC-05 and CD-02 the seller's proof verified and payment released in one transaction; RO-02 real funds required; AS-15 the buyer decrypting at the declared tier and the seller destroying at HARD.

**Stop criterion.** No priced deployment before review and legal closure; a composition or verifier finding not dispositioned halts here.

**Deliverables.** The mainnet deployment record; the proof's settlement records.

**Owner.** Project lead with the contract implementer and relayer operator.

**Unblocks.** The acceptance run's priced scenario.

### External review and legal prerequisites closed

**Purpose.** The two release conditions engineering does not control, started at the harness and closed here.

**Scope.** Cryptographic review findings on the composition claims and the verifier tracked to disposition; license text adopted and the content-terms field populated for the project's packages, LI-01; the eligibility principle and residual restated in MVP Scope; regulatory review of priced entitlements and the paymaster; export constraints on the packaged binary confirmed; code-signing certificates in hand.

**Entry.** Harness report, for review inputs; nothing else technical.

**Exit.** Every finding dispositioned and recorded in release evidence; every legal item recorded as a release-evidence document.

**Deliverables.** Release-evidence documents.

**Owner.** Project lead.

**Unblocks.** Release.

### Acceptance run

**Purpose.** The completion boundary, met or not.

**Scope.** Every acceptance scenario through the packaged applications on clean machines on every supported platform, driven by the demonstration harness against Base Sepolia, the priced scenario against the Base mainnet pilot, with every metric reconciled against induced activity and the telemetry scanned; no test-only bypass of onboarding, the package host, capability resolution, or the deployed verifier.

**Entry.** Every milestone above.

**Exit.** Every acceptance scenario passes; every guardrail holds; the requirement-to-proof reconciliation is complete.

**Stop criterion.** Any guardrail breach blocks release, and no guardrail may be waived by adjusting the metric.

**Deliverables.** The acceptance record.

**Owner.** Project lead.

**Unblocks.** Release.

### Dogfood baseline and release

**Purpose.** Ship, with the numbers that make the next decision possible.

**Scope.** The dogfood baseline for the north star, primary KPIs, and leading indicators recorded on the Base mainnet pilot alongside the harness measurements, the mainnet re-take of the L1 data fee, the chosen piece-group size, curve, and attempt-rule parameters; signed packages published; the extension and npm package published; the site live with the demonstration; the reporting cadence set; the next planning cycle's re-map of deferred items begun.

**Entry.** Acceptance run; external review and legal prerequisites closed; transaction flow proof.

**Exit.** Release evidence complete under the Application Requirements' completion boundary; both onboarding paths reaching the same postcondition from published artifacts.

**Deliverables.** The release and its evidence.

**Owner.** Project lead.

**Unblocks.** The adopting population.

# Iteration Semantics

**Resolution decays and re-maps.** The foundation and harness groupings and the hashing, signature, and swarm milestones are at ticket resolution. When the harness report node commits, the remaining protocol core milestones are re-mapped from sprints to tickets using what the harness taught and its measured throughput, the daemon grouping from epics to sprints, and so on outward; this document and the dependency map are revised in place.

**Nodes are authored from tickets through the ordinary path.** A ticket names a source file's role, dependencies, and proof; the workplan author writes the node in the fixed element order, resolving every path and symbol so the implementer instantiates rather than invents; the implementer builds it under the repository's process topics. A family's first node, its first concrete or a family-owned function preceding it, authors the family's interface and declaration as its producers, a concrete adapter is one node whose operations are its methods, the factory node follows every concrete and is revised in place when a later milestone adds one, and a consumer node depends on a factory's surface and never on a concrete.

**A milestone closes on its commit.** The last node in a milestone's chain carries the integration test across the chain and the commit; the milestone's exit rows are the requirements that test proves.

**Discovery revises the map, not the node.** An implementer who finds that a node needs a second file, a missing producer, or a decision the node did not make reports and halts; the author revises the map and the node, and the milestone's scope is restated in full, never as a delta.

**Stop criteria are evaluated where named.** At the post-harness re-map, the acceptance run, and review closure; each halts the plan at that point when met.

**Parallelism is by dependency only.** Independent milestones may run together when the team allows; the map's edges, not a calendar, decide.
# Features Context

The in-scope features of the [feature spec](feature-spec.md) map onto milestones as follows. Cryptographic services and validation harness: the harness grouping and the hashing and payload cipher milestones. Adapter composition: the daemon skeleton. Entitlement ledger and settlement: registry contracts and the chain and submission families. Swarm, discovery, and seed hosting: swarm transport and seed host, and the project seed host. Package serving and CAS: plaintext CAS and package host. Encrypted content consumption: credential delivery and per-attempt authorization. First Finder bootstrap: First Finder engine. Credential delivery: credential delivery and per-attempt authorization, and claim. Identity, wallet, and custody: identity and custody. Settings and configuration: daemon skeleton, with surfaces in CLI and desktop. Self-installing onboarding: installation coordinator, the extension and npm bootstrap. Relaying: relayer. Explicit publishing and escrow claim: explicit publisher path, claim verifier and escrow claim. Project seed host and site: its milestone. Observability: observability reconciliation. Test facilities and demonstration harness: demonstration harness and the acceptance run. Transaction flow proof: its milestone. The deferred features and the travelling-developer story constrain choices in identity and custody, the daemon skeleton's IPC, and the publisher path's issuance policy, and consume no milestone.

# Feasibility Insights

From the [feasibility assessment](technical-feasibility.md): technical feasibility high, delivery feasibility undetermined until the harness report calibrates throughput, which is why that milestone carries the re-map and the funding stop criterion. Platform breadth is the largest delivery risk and is concentrated in the installation coordinator milestone, whose scope is bounded by the decided platform matrix. The demonstration harness is load-bearing and is its own milestone. The Solidity verifier is generated from the Rust reference in the delivery-proof milestone. Transfer is proven last, in the transaction flow proof, and its external gates start at the harness so they do not arrive at the end.

# Non-Functional Alignment

Each non-functional requirement of the [review](non-functional-requirements.md) is proven inside the milestone that owns its subsystem: security rows in the harness, contracts, and credential-delivery milestones; performance rows at the harness report, the demonstrable milestone, and observability reconciliation, each reported against baseline; reliability rows in the daemon skeleton, installation coordinator, and swarm milestones; maintainability rows in the workspace bootstrap and the tickets that own each type; compliance rows in external review and legal prerequisites. The requirements the review added, XA-08, XA-09, XA-10, IC-09, IC-10, RO-07, RO-08, and LI-02, are carried where they land: signing-key custody in the bootstrap and installer; the package-host threat model in the daemon skeleton's IPC; footprint limits and cold-start in the daemon skeleton and settings; backoff bounds in the chain and swarm adapters; the suite compatibility statement at release; regulatory and export review in legal prerequisites; accessibility in CLI and desktop; the first-run cost disclosure in the installer.

# Architecture Summary

As the [system architecture](system-architecture.md) states: one canonical ciphertext per deployment and one sidecar per live parameter set, each its own object, on the swarm, entitlements on Base, credentials delivered inside settlement and verified by proof, per-attempt local authorization, and a daemon serving npm's protocol from whichever of cache, swarm, and registry delivers within the configured source deadline, never slower than the registry alone by more than the hedge delay, leaving a first run independent, with nothing on the read path.

# Services

The daemon, relayer, claim verifier, seed host and site, validation harness, and demonstration harness, each built in the milestone named above and each bounded by what it must never become.

# Components

The rings and the adapter table of the system architecture; the subsystem register of the technical requirements assigns each to its crate and, through this document, to its milestone.

# Dependency Resolution

Declaration, composition, suite binding, fail closed, and the on-chain factory, built in the daemon skeleton and the registry contracts and proven under IC-05, SI-10, and AS-17 in the acceptance run.

# Component Details

The daemon's subsystems, their ownership, trust boundaries, process model, and storage ownership are in the system architecture's daemon breakout; their APIs, schemas, and file tree are in the technical requirements. This document adds only their milestone assignment, which the Milestones section states per scope line.

# Integration Requirements

The integration points of the dependency map are each crossed by the integration test of the milestone that first joins the two sides: factory admission in the harness for the cryptographic families and in the daemon skeleton for the whole composition; verifier parity and precompile encoding in the delivery-proof milestone; the decapsulation-to-cipher seam and sidecar commitment in payload cipher and sidecar layer; the discovery engine and the store in the swarm milestone; envelope-in-settlement, the signed intent, and attestation verification on chain in registry contracts; attempt context in the chain and submission families; resolution order and the package-manager protocol in the CAS and package host; the persistent credential and device admission in identity and custody; grant request to holder and grant pickup from events in credential delivery; installer-to-daemon, artifact authentication, and shells-to-coordinator in the installer and shells; relayer-to-contracts in the relayer; voucher-to-contract in claim; the site demonstration in the seed host and site milestone.

# Migration Context

Migrations in the MVP's own lifetime, and the one beyond it. Settings and store roots migrate through checkpointed jobs under ST-02. Configuration schema migrates under IC-04 and SI-17 with reversible updates. Custody blobs migrate in place between custody concretes and versions by the custody family's checkpointed upgrade job, which verifies the re-derived public keys against the chain before retiring the old blob; a change of storage concrete migrates each store through the storage family's interface with the same checkpointing. A deployment suite changes only by a successor deployment carrying a sidecar per live parameter set, and a parameter set retires only when no live entitlement remains under it. The `librqbit` overlay migrates onto each upstream release by rebase, with the hard-fork trigger ending that track. Beyond the MVP, entitlements and contract state do not move between chain concretes, no mechanism exists, and one is required before any second chain, including the project's own, carries live entitlements; it is held in the workplan To-Do and is not scheduled here.
